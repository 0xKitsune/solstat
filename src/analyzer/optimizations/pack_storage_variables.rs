use std::collections::HashSet;

use solang_parser::pt::{self, Expression, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};
use crate::analyzer::utils;

pub fn pack_storage_variables_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let target_nodes =
        ast::extract_target_from_node(Target::ContractDefinition, source_unit.into());

    for node in target_nodes {
        let source_unit_part = node.source_unit_part().unwrap();

        if let pt::SourceUnitPart::ContractDefinition(contract_definition) = source_unit_part {
            let mut variable_sizes: Vec<u16> = vec![];

            for part in contract_definition.clone().parts {
                if let pt::ContractPart::VariableDefinition(box_variable_definition) = part {
                    variable_sizes.push(utils::get_type_size(box_variable_definition.ty));
                }
            }

            //Cache a version of variable sizes that is unordered
            let unordered_variable_sizes = variable_sizes.clone();

            //Sort the variable sizes
            variable_sizes.sort();

            //If the ordered version is smaller than the
            if utils::storage_slots_used(unordered_variable_sizes)
                > utils::storage_slots_used(variable_sizes)
            {
                optimization_locations.insert(contract_definition.loc);
            }
        }
    }

    optimization_locations
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
