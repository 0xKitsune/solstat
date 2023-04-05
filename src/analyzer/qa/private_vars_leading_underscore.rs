use std::collections::HashSet;

use solang_parser::pt::{self, FunctionTy, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};
use crate::analyzer::utils::get_32_byte_storage_variables;

pub fn private_vars_leading_underscore(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each qa target identified
    let mut qa_locations: HashSet<Loc> = HashSet::new();

    let storage_variables = get_32_byte_storage_variables(source_unit.clone(), true, false);

    for (variable_name, variable_attribute) in storage_variables {
        let (option_variable_attributes, loc) = variable_attribute;

        if option_variable_attributes.is_some() {
            let variable_attributes = option_variable_attributes.unwrap();

            for attr in variable_attributes {
                if let pt::VariableAttribute::Visibility(v) = attr {
                    match v {
                        pt::Visibility::Private(_) | pt::Visibility::Internal(_) => {
                            if !variable_name.starts_with('_') {
                                qa_locations.insert(loc);
                            }
                        }
                        // Public variables
                        _ => {
                            if variable_name.starts_with('_') {
                                qa_locations.insert(loc);
                            }
                        }
                    }
                }
            }
        }
    }

    //Return the identified qa locations
    qa_locations
}

#[test]
fn test_private_vars_leading_underscore() {
    let file_contents = r#"
    
    contract Contract0 {
        address public addr1;
        address public _addr2;
        address private _addr3;
        address private addr4;
        address internal _addr5;
        address internal addr6;
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let qa_locations = private_vars_leading_underscore(source_unit);
    assert_eq!(qa_locations.len(), 3)
}
