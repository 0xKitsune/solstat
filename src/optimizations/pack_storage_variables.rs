use std::collections::HashSet;

use solang_parser::pt::{self, Expression, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::ast::ast::{self, Target};

pub fn pack_storage_variables_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let target_nodes =
        ast::extract_target_from_node(Target::ContractDefinition, source_unit.into());

    for node in target_nodes {
        let source_unit_part = node
            .source_unit_part()
            .expect("Node is not a source unit part");

        if let pt::SourceUnitPart::ContractDefinition(contract_definition) = source_unit_part {
            let mut variable_sizes: Vec<u16> = vec![];

            for part in contract_definition.clone().parts {
                if let pt::ContractPart::VariableDefinition(box_variable_definition) = part {
                    variable_sizes.push(get_type_size(box_variable_definition.ty));
                }
            }

            //Cache a version of variable sizes that is unordered
            let unordered_variable_sizes = variable_sizes.clone();

            //Sort the variable sizes
            variable_sizes.sort();

            //If the ordered version is smaller than the
            if storage_slot_size(unordered_variable_sizes) > storage_slot_size(variable_sizes) {
                optimization_locations.insert(contract_definition.loc);
            }
        }
    }

    optimization_locations
}

fn storage_slot_size(variables: Vec<u16>) -> u32 {
    //set a variable to keep track of how many bytes have been used in the slot
    let mut bytes_used_in_slot = 0;
    //--------------------- test slot usage of unordered variable sizes ---------------------------------------

    //loop through the unordered variable sizes and count the amount of slots used
    let mut slots_used = 0;
    for variable_size in variables {
        //if the next variable size
        if bytes_used_in_slot + variable_size > 256 {
            //add a slot used
            slots_used += 1;

            //update bytes used in slot
            bytes_used_in_slot = variable_size;
        } else {
            bytes_used_in_slot += variable_size;
        }
    }

    //if the bytes in slot is > 0 and the last variable has been accounted for, add one more slot used
    if bytes_used_in_slot > 0 {
        slots_used += 1;
    }

    slots_used
}

///Returns the size of the type in bytes
fn get_type_size(expression: Expression) -> u16 {
    if let Expression::Type(_, ty) = expression {
        match ty {
            pt::Type::Address => return 256,
            pt::Type::AddressPayable => return 256,
            pt::Type::Bytes(_size) => return (_size as u16) * 4,
            pt::Type::Bool => return 1,
            pt::Type::Int(_size) => return _size,
            pt::Type::Uint(_size) => return _size,
            _ => return 256,
        }
    }

    256
}

#[test]
fn test_pack_storage_variables_optimization() {
    let file_contents = r#"

contract Contract0 {
  uint256 num0;
  uint256 num1;
  bool bool0;
  uint256 num2;
  bool bool1;

}

contract Contract1 {
    uint256 num0;
    uint256 num1;
    uint256 num2;
    bool bool0;
    bool bool1;
  }

    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = pack_storage_variables_optimization(source_unit);

    assert_eq!(optimization_locations.len(), 1)
}
