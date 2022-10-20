use std::collections::{HashMap, HashSet};

use solang_parser::pt::{ContractPart, Expression, Loc, StructDefinition, Type};
use solang_parser::{self, pt::SourceUnit, pt::SourceUnitPart};

use crate::ast::ast::{self, Target};
use crate::utils;

///Identifiy opportunities to pack structs to save gas
pub fn pack_struct_variables_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let target_nodes = ast::extract_targets_from_node(
        //Contract definition and Struct definition are targets since a struct definition can
        //be inside or outside of the contract definition
        vec![Target::ContractDefinition, Target::StructDefinition],
        source_unit.into(),
    );

    for node in target_nodes {
        if node.is_source_unit_part() {
            if let SourceUnitPart::ContractDefinition(contract_definition) =
                node.source_unit_part().unwrap()
            {
                for part in contract_definition.parts {
                    if let ContractPart::StructDefinition(struct_definition) = part {
                        let struct_location = struct_definition.loc;
                        if struct_can_be_packed(*struct_definition) {
                            optimization_locations.insert(struct_location);
                        }
                    }
                }
            }
        } else if node.is_contract_part() {

            //TODO:
        }
    }
    optimization_locations
}

fn struct_can_be_packed(struct_definition: StructDefinition) -> bool {
    let mut variable_sizes: Vec<u16> = vec![];

    for variable_declaration in struct_definition.fields {
        variable_sizes.push(utils::get_type_size(variable_declaration.ty));
    }

    //create an unordered list of variable sizes
    let unordered_variable_sizes = variable_sizes.clone();

    //Sort the variable sizes
    variable_sizes.sort();

    //If the ordered version is smaller than the
    if utils::storage_slots_used(unordered_variable_sizes)
        > utils::storage_slots_used(variable_sizes)
    {
        true
    } else {
        false
    }
}

#[test]
fn test_pack_struct_variables_optimization() {
    let file_contents = r#"

    //should not match
    struct Ex {
        uint256 spotPrice;
        uint128 res0;
        uint128 res1;
    }
    

contract OrderRouter {
  
    
    //should match
    struct Ex1 {
        bool isUniV2;
        address factoryAddress;
        bytes16 initBytecode;
    }

    //should not match
    struct Ex2 {
        bytes16 initBytecode;
        bool isUniV2;
        address factoryAddress;
    }

    //should not match
    struct Ex3 {
        bool isUniV2;
        bytes16 initBytecode;
        address factoryAddress;
    }

    //should match
    struct Ex4 {
        uint128 thing3;
        uint256 thing1;
        uint128 thing2;
    }
  

}
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = pack_struct_variables_optimization(source_unit);

    assert_eq!(optimization_locations.len(), 2)
}
