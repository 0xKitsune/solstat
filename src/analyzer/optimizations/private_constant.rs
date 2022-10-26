use std::collections::HashSet;

use solang_parser::pt::{self, Loc, SourceUnit};

use crate::analyzer::utils;

pub fn private_constant_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let storage_variables = utils::get_32_byte_storage_variables(source_unit.clone(), false, true);

    for (_, variable_data) in storage_variables {
        let (option_variable_attributes, loc) = variable_data;

        if option_variable_attributes.is_some() {
            let variable_attributes = option_variable_attributes.unwrap();

            let mut is_constant = false;
            let mut is_private = false;

            for variable_attribute in variable_attributes {
                match variable_attribute {
                    pt::VariableAttribute::Constant(_) => {
                        is_constant = true;
                    }

                    pt::VariableAttribute::Visibility(visibility) => match visibility {
                        pt::Visibility::Private(_) => is_private = true,
                        _ => {}
                    },

                    _ => {}
                }
            }

            if is_constant && !is_private {
                optimization_locations.insert(loc);
            }
        }
    }

    optimization_locations
}

#[test]
fn test_private_constant_optimization() {
    let file_contents = r#"
    
contract Contract0 {

    uint256 constant public x = 100;
    uint256 constant private y = 100;
    uint256 constant z = 100;


    function addPublicConstant(uint256 a) external pure returns (uint256) {
        return a + x;
    }


    function addPrivateConstant(uint256 a) external pure returns (uint256) {
        return a +x;
    }
}

    "#;
    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;
    let optimization_locations = private_constant_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 2)
}
