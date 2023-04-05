use std::collections::{HashMap, HashSet};

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};
use crate::analyzer::utils::get_32_byte_storage_variables;

pub fn immutable_variables_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    //Get all storage variables that are not marked constant or immutable
    let storage_variables = get_32_byte_storage_variables(source_unit.clone(), true, true);

    let mut potential_immutable_variables =
        get_storage_variables_assigned_in_constructor(source_unit.clone(), storage_variables);

    let contract_definition_nodes =
        ast::extract_target_from_node(Target::ContractDefinition, source_unit.clone().into());

    for contract_definition_node in contract_definition_nodes {
        let target_nodes = ast::extract_target_from_node(
            Target::FunctionDefinition,
            contract_definition_node.clone().into(),
        );

        for node in target_nodes {
            //Can unwrap since Target::FunctionDefinition inside a contract definition will always be a contract part
            let contract_part = node.contract_part().unwrap();

            if let pt::ContractPart::FunctionDefinition(box_function_definition) =
                contract_part.clone()
            {
                if let pt::FunctionTy::Constructor = box_function_definition.ty {
                } else {
                    //Extract the target nodes from the function definitions
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
                        contract_part.into(),
                    );

                    //For each target node that was extracted, check for the optimization patterns
                    for node in target_nodes {
                        let expression = node.expression().unwrap();
                        match expression {
                            pt::Expression::Assign(_, box_expression, _) => {
                                if let pt::Expression::Variable(identifier) = *box_expression {
                                    //if the variable name exists in the storage variable hashmap
                                    if potential_immutable_variables.contains_key(&identifier.name)
                                    {
                                        //if the variable has been used, remove it from storage variables
                                        potential_immutable_variables.remove(&identifier.name);
                                    }
                                }
                            }
                            pt::Expression::PreIncrement(_, box_expression) => {
                                if let pt::Expression::Variable(identifier) = *box_expression {
                                    //if the variable name exists in the storage variable hashmap
                                    if potential_immutable_variables.contains_key(&identifier.name)
                                    {
                                        //if the variable has been used, remove it from storage variables
                                        potential_immutable_variables.remove(&identifier.name);
                                    }
                                }
                            }
                            pt::Expression::PostIncrement(_, box_expression) => {
                                if let pt::Expression::Variable(identifier) = *box_expression {
                                    //if the variable name exists in the storage variable hashmap
                                    if potential_immutable_variables.contains_key(&identifier.name)
                                    {
                                        //if the variable has been used, remove it from storage variables
                                        potential_immutable_variables.remove(&identifier.name);
                                    }
                                }
                            }
                            pt::Expression::PreDecrement(_, box_expression) => {
                                if let pt::Expression::Variable(identifier) = *box_expression {
                                    //if the variable name exists in the storage variable hashmap
                                    if potential_immutable_variables.contains_key(&identifier.name)
                                    {
                                        //if the variable has been used, remove it from storage variables
                                        potential_immutable_variables.remove(&identifier.name);
                                    }
                                }
                            }
                            pt::Expression::PostDecrement(_, box_expression) => {
                                if let pt::Expression::Variable(identifier) = *box_expression {
                                    //if the variable name exists in the storage variable hashmap
                                    if potential_immutable_variables.contains_key(&identifier.name)
                                    {
                                        //if the variable has been used, remove it from storage variables
                                        potential_immutable_variables.remove(&identifier.name);
                                    }
                                }
                            }

                            pt::Expression::AssignAdd(_, box_expression, _) => {
                                if let pt::Expression::Variable(identifier) = *box_expression {
                                    //if the variable name exists in the storage variable hashmap
                                    if potential_immutable_variables.contains_key(&identifier.name)
                                    {
                                        //if the variable has been used, remove it from storage variables
                                        potential_immutable_variables.remove(&identifier.name);
                                    }
                                }
                            }
                            pt::Expression::AssignAnd(_, box_expression, _) => {
                                if let pt::Expression::Variable(identifier) = *box_expression {
                                    //if the variable name exists in the storage variable hashmap
                                    if potential_immutable_variables.contains_key(&identifier.name)
                                    {
                                        //if the variable has been used, remove it from storage variables
                                        potential_immutable_variables.remove(&identifier.name);
                                    }
                                }
                            }
                            pt::Expression::AssignDivide(_, box_expression, _) => {
                                if let pt::Expression::Variable(identifier) = *box_expression {
                                    //if the variable name exists in the storage variable hashmap
                                    if potential_immutable_variables.contains_key(&identifier.name)
                                    {
                                        //if the variable has been used, remove it from storage variables
                                        potential_immutable_variables.remove(&identifier.name);
                                    }
                                }
                            }
                            pt::Expression::AssignModulo(_, box_expression, _) => {
                                if let pt::Expression::Variable(identifier) = *box_expression {
                                    //if the variable name exists in the storage variable hashmap
                                    if potential_immutable_variables.contains_key(&identifier.name)
                                    {
                                        //if the variable has been used, remove it from storage variables
                                        potential_immutable_variables.remove(&identifier.name);
                                    }
                                }
                            }
                            pt::Expression::AssignMultiply(_, box_expression, _) => {
                                if let pt::Expression::Variable(identifier) = *box_expression {
                                    //if the variable name exists in the storage variable hashmap
                                    if potential_immutable_variables.contains_key(&identifier.name)
                                    {
                                        //if the variable has been used, remove it from storage variables
                                        potential_immutable_variables.remove(&identifier.name);
                                    }
                                }
                            }
                            pt::Expression::AssignOr(_, box_expression, _) => {
                                if let pt::Expression::Variable(identifier) = *box_expression {
                                    //if the variable name exists in the storage variable hashmap
                                    if potential_immutable_variables.contains_key(&identifier.name)
                                    {
                                        //if the variable has been used, remove it from storage variables
                                        potential_immutable_variables.remove(&identifier.name);
                                    }
                                }
                            }
                            pt::Expression::AssignShiftLeft(_, box_expression, _) => {
                                if let pt::Expression::Variable(identifier) = *box_expression {
                                    //if the variable name exists in the storage variable hashmap
                                    if potential_immutable_variables.contains_key(&identifier.name)
                                    {
                                        //if the variable has been used, remove it from storage variables
                                        potential_immutable_variables.remove(&identifier.name);
                                    }
                                }
                            }
                            pt::Expression::AssignShiftRight(_, box_expression, _) => {
                                if let pt::Expression::Variable(identifier) = *box_expression {
                                    //if the variable name exists in the storage variable hashmap
                                    if potential_immutable_variables.contains_key(&identifier.name)
                                    {
                                        //if the variable has been used, remove it from storage variables
                                        potential_immutable_variables.remove(&identifier.name);
                                    }
                                }
                            }
                            pt::Expression::AssignSubtract(_, box_expression, _) => {
                                if let pt::Expression::Variable(identifier) = *box_expression {
                                    //if the variable name exists in the storage variable hashmap
                                    if potential_immutable_variables.contains_key(&identifier.name)
                                    {
                                        //if the variable has been used, remove it from storage variables
                                        potential_immutable_variables.remove(&identifier.name);
                                    }
                                }
                            }
                            pt::Expression::AssignXor(_, box_expression, _) => {
                                if let pt::Expression::Variable(identifier) = *box_expression {
                                    //if the variable name exists in the storage variable hashmap
                                    if potential_immutable_variables.contains_key(&identifier.name)
                                    {
                                        //if the variable has been used, remove it from storage variables
                                        potential_immutable_variables.remove(&identifier.name);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    //if the variable is not been reassigned, add it to the optimization locations
    for variable in potential_immutable_variables {
        optimization_locations.insert(variable.1);
    }

    //Return the identified optimization locations
    optimization_locations
}

pub fn get_storage_variables_assigned_in_constructor(
    source_unit: SourceUnit,
    storage_variables: HashMap<String, (Option<Vec<pt::VariableAttribute>>, Loc)>,
) -> HashMap<String, Loc> {
    let mut potential_immutable_variables: HashMap<String, Loc> = HashMap::new();

    let contract_definition_nodes =
        ast::extract_target_from_node(Target::ContractDefinition, source_unit.clone().into());

    for contract_definition_node in contract_definition_nodes {
        let target_nodes = ast::extract_target_from_node(
            Target::FunctionDefinition,
            contract_definition_node.clone().into(),
        );

        for node in target_nodes {
            //Can unwrap since Target::FunctionDefinition inside a contract definition will always be a contract part
            let contract_part = node.contract_part().unwrap();

            if let pt::ContractPart::FunctionDefinition(box_function_definition) = contract_part {
                if let pt::FunctionTy::Constructor = box_function_definition.ty {
                    let target_nodes =
                        ast::extract_target_from_node(Target::Assign, source_unit.clone().into());

                    for node in target_nodes {
                        //Can unwrap since Target::Assign will always be an expression
                        let expression = node.expression().unwrap();
                        if let pt::Expression::Assign(_, box_expression, box_assigned_value) =
                            expression
                        {
                            /*
                             * A Non-Value Type can not be immutable
                             * https://docs.soliditylang.org/en/v0.8.13/contracts.html?highlight=immutable#constant-and-immutable-state-variables
                             */
                            if is_a_non_value_type(box_assigned_value) {
                                continue;
                            }

                            //if the first expr in the assign expr is a variable
                            if let pt::Expression::Variable(identifier) = *box_expression {
                                //if the variable name exists in the storage variable hashmap
                                if storage_variables.contains_key(&identifier.name) {
                                    let storage_var = storage_variables.get(&identifier.name);

                                    if storage_var.is_some() {
                                        let loc = storage_var.unwrap().1;
                                        //add the variable to the variable usage map
                                        potential_immutable_variables.insert(identifier.name, loc);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    potential_immutable_variables
}

fn is_a_non_value_type(assigned_value: Box<pt::Expression>) -> bool {
    match *assigned_value {
        // string types
        pt::Expression::StringLiteral(_) => return true,
        // Dynamic bytes
        pt::Expression::FunctionCall(_, box_fn_call, _) => {
            // bytes (ex: bytes name = abi.encode("Vitalik"))
            if let pt::Expression::MemberAccess(_, box_member_access_variable, _) =
                *box_fn_call.clone()
            {
                if let pt::Expression::Variable(member_access_identifier) =
                    *box_member_access_variable
                {
                    return member_access_identifier.name == "abi";
                }
            }

            // bytes (ex: bytes name = bytes("Vitalik"))
            if let pt::Expression::Type(_, ty) = *box_fn_call {
                return pt::Type::DynamicBytes == ty;
            }
        }
        _ => (),
    }

    return false;
}

#[test]
fn test_immutable_variables_optimization() {
    let file_contents = r#"
    
    pragma solidity >= 0.8.0;
    contract Contract {


        uint256 immutable num0;
        uint256 num1;
        uint256 num2;
        address addr1 = address(0);
        string str1;
        string str2;
        bytes b1;
        bytes b2;
        bytes b3;


        constructor(){
            num1 = 100;
            num2 = 100;
            str1 = "Test Name";
            str2 = "Another test content";
            b1 = abi.encode("Test content");
            b2 = abi.encodePacked("Test content");
            b3 = bytes("Vitalik");
        }

       
        function testFunction() public {
            addr1 = address(0);
            uint256 thing = num1;
            str2 = "i can no longer be immutable anymore";
        }
    }
 
    "#;
    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = immutable_variables_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 2)
}
