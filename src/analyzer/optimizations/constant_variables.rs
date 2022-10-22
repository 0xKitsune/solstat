use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};
use crate::analyzer::utils;

pub fn constant_variable_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let mut storage_variables =
        utils::get_32_byte_storage_variables(source_unit.clone(), true, false);

    let target_nodes = ast::extract_targets_from_node(
        vec![
            Target::Assign,
            Target::PreIncrement,
            Target::PostIncrement,
            Target::PreDecrement,
            Target::PostDecrement,
            Target::AssignAdd,
            Target::AssignAnd,
            Target::AssignDivide,
            Target::AssignModulo,
            Target::AssignMultiply,
            Target::AssignOr,
            Target::AssignShiftLeft,
            Target::AssignShiftRight,
            Target::AssignSubtract,
            Target::AssignXor,
        ],
        source_unit.into(),
    );

    for node in target_nodes {
        let expression = node.expression().unwrap();

        match expression {
            pt::Expression::Assign(_, box_expression, _) => {
                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        //if the variable has been used, remove it from storage variables
                        storage_variables.remove(&identifier.name);
                    }
                }
            }
            pt::Expression::PreIncrement(_, box_expression) => {
                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        //if the variable has been used, remove it from storage variables
                        storage_variables.remove(&identifier.name);
                    }
                }
            }
            pt::Expression::PostIncrement(_, box_expression) => {
                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        //if the variable has been used, remove it from storage variables
                        storage_variables.remove(&identifier.name);
                    }
                }
            }
            pt::Expression::PreDecrement(_, box_expression) => {
                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        //if the variable has been used, remove it from storage variables
                        storage_variables.remove(&identifier.name);
                    }
                }
            }
            pt::Expression::PostDecrement(_, box_expression) => {
                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        //if the variable has been used, remove it from storage variables
                        storage_variables.remove(&identifier.name);
                    }
                }
            }

            pt::Expression::AssignAdd(_, box_expression, _) => {
                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        //if the variable has been used, remove it from storage variables
                        storage_variables.remove(&identifier.name);
                    }
                }
            }
            pt::Expression::AssignAnd(_, box_expression, _) => {
                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        //if the variable has been used, remove it from storage variables
                        storage_variables.remove(&identifier.name);
                    }
                }
            }
            pt::Expression::AssignDivide(_, box_expression, _) => {
                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        //if the variable has been used, remove it from storage variables
                        storage_variables.remove(&identifier.name);
                    }
                }
            }
            pt::Expression::AssignModulo(_, box_expression, _) => {
                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        //if the variable has been used, remove it from storage variables
                        storage_variables.remove(&identifier.name);
                    }
                }
            }
            pt::Expression::AssignMultiply(_, box_expression, _) => {
                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        //if the variable has been used, remove it from storage variables
                        storage_variables.remove(&identifier.name);
                    }
                }
            }
            pt::Expression::AssignOr(_, box_expression, _) => {
                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        //if the variable has been used, remove it from storage variables
                        storage_variables.remove(&identifier.name);
                    }
                }
            }
            pt::Expression::AssignShiftLeft(_, box_expression, _) => {
                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        //if the variable has been used, remove it from storage variables
                        storage_variables.remove(&identifier.name);
                    }
                }
            }
            pt::Expression::AssignShiftRight(_, box_expression, _) => {
                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        //if the variable has been used, remove it from storage variables
                        storage_variables.remove(&identifier.name);
                    }
                }
            }
            pt::Expression::AssignSubtract(_, box_expression, _) => {
                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        //if the variable has been used, remove it from storage variables
                        storage_variables.remove(&identifier.name);
                    }
                }
            }
            pt::Expression::AssignXor(_, box_expression, _) => {
                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        //if the variable has been used, remove it from storage variables
                        storage_variables.remove(&identifier.name);
                    }
                }
            }
            _ => {}
        }
    }

    //if the variable is not been reassigned, add it to the optimization locations
    for variable in storage_variables {
        optimization_locations.insert(variable.1 .1);
    }

    optimization_locations
}

#[test]
fn test_constant_variable_optimization() {
    let file_contents = r#"
    
    pragma solidity >= 0.8.0;
    contract Contract {


        uint256 firstUint256 = 0;
        uint256 secondUint256 = 100;
        uint256 immutable thirdUint256 = 100;
        uint256 fourthUint256 = 100;
        uint256 constant fifthUint256 = 1000000;

       
        function testFunction() public {
            firstUint256 = 10;
            secondUint256 = someVal;
        }
    }
 
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = constant_variable_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 2)
}
