use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
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

            //If the ordered version is smaller than the unordered, add loc
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
    // Optimal packing
    let contract = r#"
    contract Contract {
        uint256 num0;
        uint256 num1;
        uint256 num2;
        bool bool0;
        bool bool1;
    }
    "#;

    let source_unit = solang_parser::parse(contract, 0).unwrap().0;
    let optimization_locations = pack_storage_variables_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 0);

    // Cannot pack better, 2x bytes24 don't fit in a slot
    let contract = r#"
        contract Contract {
            bytes24 b0;
            uint256 num0;
            bytes24 b1;
        }
        "#;

    let source_unit = solang_parser::parse(contract, 0).unwrap().0;
    let optimization_locations = pack_storage_variables_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 0);

    // Cannot pack better, bool are stored with uint8 so cannot move bo1
    let contract = r#"
        contract Contract {
            bytes28 b0;
            uint8 num0;
            uint8 num1;
            uint8 num2;
            bool bo0;
            uint256 num3;
            bool bo1;
        }
        "#;

    let source_unit = solang_parser::parse(contract, 0).unwrap().0;
    let optimization_locations = pack_storage_variables_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 0);

    // Suboptimal, can be packed better
    let contract = r#"
    contract Contract {
        uint256 num0;
        uint256 num1;
        bool bool0;
        uint256 num2;
        bool bool1;
    }
    "#;

    let source_unit = solang_parser::parse(contract, 0).unwrap().0;
    let optimization_locations = pack_storage_variables_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 1);

    // Suboptimal, can be packed better (owner,bool,num0);
    let contract = r#"
    contract Contract {
        address owner;
        uint256 num0;
        bool bool0;
    }
    "#;

    let source_unit = solang_parser::parse(contract, 0).unwrap().0;
    let optimization_locations = pack_storage_variables_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 1);

    // Suboptimal, can be packed better (owner,num1,b0,num0)
    let contract = r#"
        contract Contract {
            address owner; // 160 bits
            uint256 num0;  // 256 bits
            bytes4 b0;     // 32 bits
            uint64 num1;   // 64 bits
        }
        "#;

    let source_unit = solang_parser::parse(contract, 0).unwrap().0;
    let optimization_locations = pack_storage_variables_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 1);
}
