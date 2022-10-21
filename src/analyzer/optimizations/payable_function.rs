use std::collections::HashSet;

use solang_parser::pt::{self, ContractPart, Expression, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};

pub fn payable_function_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let target_nodes =
        ast::extract_target_from_node(Target::FunctionDefinition, source_unit.into());

    for node in target_nodes {
        //We can use unwrap because Target::FunctionDefinition is a contract_part
        let contract_part = node.contract_part().unwrap();

        if let pt::ContractPart::FunctionDefinition(function_definition) = contract_part {
            //if there is function body
            if function_definition.body.is_some() {
                if function_definition.attributes.len() > 0 {
                    let mut payable = false;
                    let mut public_or_external = false;

                    for attr in function_definition.attributes {
                        match attr {
                            // Visi
                            pt::FunctionAttribute::Visibility(visibility) => match visibility {
                                pt::Visibility::External(_) => {
                                    public_or_external = true;
                                }
                                pt::Visibility::Public(_) => {
                                    public_or_external = true;
                                }
                                _ => {}
                            },
                            pt::FunctionAttribute::Mutability(mutability) => {
                                if let pt::Mutability::Payable(_) = mutability {
                                    payable = true;
                                }
                            }
                            _ => {}
                        }
                    }

                    //if the function is public or external, and it is not marked as payable
                    if public_or_external && !payable {
                        //insert the loc of the function definition into optimization locations
                        optimization_locations.insert(function_definition.loc);
                    }
                }
            }
        }
    }

    optimization_locations
}

#[test]
fn test_payable_function_optimization() {
    let file_contents = r#"
    

    contract Contract0 {

        function div2(uint256 a, uint256 b) public pure {
            
        }

        function mul2(uint256 a, uint256 b) external view {
            
        }
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = payable_function_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 2)
}
