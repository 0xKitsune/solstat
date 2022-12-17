use std::collections::HashSet;

use solang_parser::pt::{
    Base, ContractPart, Expression, FunctionAttribute, FunctionDefinition, FunctionTy, Identifier,
    IdentifierPath, Loc, SourceUnit, Visibility,
};

use crate::analyzer::ast::{self, Target};

pub fn _unprotected_selfdestruct_vulnerability(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each vulnerability target identified
    let mut vulnerability_locations: HashSet<Loc> = HashSet::new();

    // what this vulnerability check does:
    // 1. extract all function definitions
    // 2. remove constructors
    // 3. take only public or external functions
    // 4. check if there are any "suicide" or "selfdestruct" calls inside
    // 5. implement an "is_protected" helper that checks for modifiers and conditions
    //   5.1. check if there are any modifiers which contain "only" in the name
    //        (suggesting that some protection has been devised, and assuming it is safe)
    //   5.2. if there aren't, check for conditions:
    //      5.2.1. either where msg.sender is used as argument (e.g. `check(msg.sender)`)
    //      5.2.2. or where msg.sender is used in a condition (e.g. `msg.sender == owner`)
    // 6. add loc of the nodes that pass all the criteria to the vulnerability_locations

    let contract_definition_nodes =
        ast::extract_target_from_node(Target::ContractDefinition, source_unit.into());

    for contract_definition_node in contract_definition_nodes {
        let target_nodes = ast::extract_target_from_node(
            Target::FunctionDefinition,
            contract_definition_node.into(),
        );

        for node in target_nodes {
            //We can use unwrap because Target::FunctionDefinition is a contract_part
            let contract_part = node.contract_part().unwrap();

            if let ContractPart::FunctionDefinition(box_function_definition) = contract_part {
                //If there is function body
                if box_function_definition.body.is_some() {
                    //Skip the constructor as it cannot be affected
                    if box_function_definition.ty == FunctionTy::Constructor {
                        continue;
                    }

                    if box_function_definition.attributes.len() > 0 {
                        //Skip functions that are not public or external as they cannot be affected
                        if !_is_public_or_external(&box_function_definition) {
                            continue;
                        }

                        let function_body_nodes = ast::extract_target_from_node(
                            Target::FunctionCall,
                            box_function_definition.body.clone().unwrap().into(),
                        );

                        for function_body_node in function_body_nodes {
                            //We can use unwrap because Target::FunctionCall is an expression
                            let expression = function_body_node.expression().unwrap();

                            if let Expression::FunctionCall(loc, box_identifier, ..) = expression {
                                //If the function is a selfdestruct call
                                if _is_selfdestruct(box_identifier) {
                                    //Check if the function is protected. If it isn't, add the loc to the vulnerabilities
                                    if !_is_protected(&box_function_definition) {
                                        vulnerability_locations.insert(loc);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    //Return the identified vulnerability locations
    vulnerability_locations
}

//Return true if the visibility of a given function is public or external. Return false otherwise.
fn _is_public_or_external(function_definition: &Box<FunctionDefinition>) -> bool {
    let mut public_or_external = false;
    if function_definition.attributes.len() > 0 {
        for attr in &function_definition.attributes {
            match attr {
                FunctionAttribute::Visibility(visibility) => match visibility {
                    Visibility::External(_) => {
                        public_or_external = true;
                    }
                    Visibility::Public(_) => {
                        public_or_external = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    public_or_external
}

fn _is_selfdestruct(box_identifier: Box<Expression>) -> bool {
    let mut is_selfdestruct = false;
    if let Expression::Variable(identifier) = *box_identifier {
        //If the function name is "selfdestruct" or "suicide"
        if identifier.name == "selfdestruct" || identifier.name == "suicide" {
            is_selfdestruct = true;
        }
    }

    is_selfdestruct
}

//Check if a function is protected using modifiers or conditions. Return false otherwise.
//This check is not exhaustive and can be improved. For example, it does not check if the modifier
//is implemented correctly. It only checks if the modifier name contains the word "only".
//Otherwise, only checks if there are any conditions on "msg.sender" applied.
fn _is_protected(function_definition: &Box<FunctionDefinition>) -> bool {
    if function_definition.attributes.len() > 0 {
        for attr in &function_definition.attributes {
            match attr {
                //If the function has a modifier
                FunctionAttribute::BaseOrModifier(_, base) => {
                    let Base { name, .. } = base;
                    let IdentifierPath { identifiers, .. } = name;

                    for identifier in identifiers {
                        //If the modifier name contains "only"
                        if identifier.name.contains("only") {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    //If there are no modifiers, check if there are any conditions applied on msg.sender
    if function_definition.body.is_some() {
        let function_body_nodes = ast::extract_target_from_node(
            Target::FunctionCall,
            function_definition.body.clone().unwrap().into(),
        );

        for node in function_body_nodes {
            //We can use unwrap because Target::MemberAccess is an expression
            let expression = node.expression().unwrap();

            if let Expression::FunctionCall(_, box_identifier, vec_expression) = expression {
                if _is_selfdestruct(box_identifier) {
                    continue;
                }

                for expression in vec_expression {
                    match expression {
                        Expression::Equal(_, box_expression, _) => {
                            if let Expression::MemberAccess(_, box_expression, identifier) =
                                *box_expression
                            {
                                //If the member access identifier is "msg.sender" the function is considered protected
                                let Identifier { name: right, .. } = identifier;
                                if let Expression::Variable(Identifier { name: left, .. }) =
                                    *box_expression
                                {
                                    if left == "msg" && right == "sender" {
                                        return true;
                                    }
                                }
                            }
                        }

                        _ => {}
                    };
                }
            }
        }
    }

    return false;
}

#[test]
fn test_unprotected_selfdestruct_vulnerability() {
    let file_contents = r#"
    
    contract Contract0 {
        // unsafe
        function unprotectedKill() public {
            selfdestruct(msg.sender);
        }

        // unsafe
        function unprotectedKill2() external {
            suicide(owner);
        }

        // safe
        function protectedKill() public {
            require(msg.sender == owner);
            selfdestruct(msg.sender);
        }

        // safe
        function protectedKill2() public onlyOwner {
            selfdestruct(msg.sender);
        }

        // safe
        function internalKill() internal {
            selfdestruct(msg.sender);
        }
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let vulnerability_locations = _unprotected_selfdestruct_vulnerability(source_unit);
    assert_eq!(vulnerability_locations.len(), 2)
}
