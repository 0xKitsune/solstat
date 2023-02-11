use std::collections::HashSet;

use solang_parser::pt::{self, Expression, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};

pub fn arbitrary_from_in_transferfrom_vulnerability(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each vulnerability target identified
    let mut vulnerability_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let target_nodes =
        ast::extract_target_from_node(Target::FunctionDefinition, source_unit.into());

    //For each target node that was extracted, check for the vulnerability patterns
    for node in target_nodes {
        let contract_part = node.contract_part().unwrap();

        if let pt::ContractPart::FunctionDefinition(box_fn_definition) = contract_part {
            // if function has no params or no body, it is probably not affected by this vulnerability.
            if box_fn_definition.params.is_empty() || box_fn_definition.body.is_none() {
                continue;
            }

            // We extract the function parameters to later verify if they are used as the first parameter in the `transferFrom/SafeTransferFrom`.
            let fn_params = extract_fn_params_as_string(&box_fn_definition);

            if let pt::Statement::Block {
                loc: _,
                unchecked: _,
                statements,
            } = box_fn_definition.body.unwrap()
            {
                // We loop through each body expression to determine if 'transferFrom/SafeTransferFrom' is used.
                for statement in statements {
                    if let pt::Statement::Expression(_, box_expression) = statement.clone() {
                        if let pt::Expression::FunctionCall(
                            loc,
                            box_fn_expression,
                            fn_call_params,
                        ) = box_expression
                        {
                            // If a call exists and any of the function parameters is used as the first parameter in the call, we mark it as a vulnerability.
                            if have_transfer_from_expression(box_fn_expression) {
                                if transfer_from_uses_fn_params_as_from_variable(
                                    &fn_params,
                                    &fn_call_params[0],
                                ) {
                                    vulnerability_locations.insert(loc);
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

fn extract_fn_params_as_string(box_fn_definition: &Box<pt::FunctionDefinition>) -> Vec<String> {
    return box_fn_definition
        .params
        .iter()
        .filter(|v| v.clone().1.is_some())
        .map(|v| v.clone().1.unwrap())
        .filter(|v| v.name.is_some())
        .map(|v| v.name.unwrap().name)
        .collect();
}

fn have_transfer_from_expression(box_fn_expression: Box<pt::Expression>) -> bool {
    const TRANSFER_FROM: &str = "safetransferfrom";
    const SAFE_TRANSFER_FROM: &str = "transferfrom";

    if let pt::Expression::MemberAccess(_, _, member_identifier) = *box_fn_expression {
        let name = member_identifier.name.to_lowercase();
        return name == TRANSFER_FROM || name == SAFE_TRANSFER_FROM;
    }

    return false;
}

fn transfer_from_uses_fn_params_as_from_variable(
    fn_params: &Vec<String>,
    from_param: &Expression,
) -> bool {
    if let pt::Expression::Variable(var_identifier) = from_param {
        // If any funtion parameter is used as `from` paramenter
        return fn_params.contains(&var_identifier.name);
    }
    return false;
}

#[test]
fn testarbitrary_from_in_transferfrom_vulnerability() {
    let file_contents = r#"
    
    contract Contract0 {
        function a(address from, address to, uint256 amount) public {
            erc20.transferFrom(from, to, amount);
        }

        function b(address to, uint256 amount) public {
            erc20.transferFrom(msg.sender, to, amount);
        }

        function c(address _from, uint256 amount) public {
            erc20.transferFrom(_from, msg.sender, amount);
        }

        function d(address _from, address _to, uint256 amount) public {
            erc20.safeTransferFrom(_from, _to, amount);
        }

        function e() public {
            erc20.safeTransferFrom(msg.sender, address(this), amount);
        }

        function f(address _token, address _sender, uint256 amount) public {
            _token.transferFrom(_sender, address(this), amount);
        }
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let vulnerability_locations = arbitrary_from_in_transferfrom_vulnerability(source_unit);
    assert_eq!(vulnerability_locations.len(), 4)
}
