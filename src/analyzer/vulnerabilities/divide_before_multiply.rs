use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{extract_targets_from_node, Target};

pub fn divide_before_multiply_vulnerability(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each vulnerability target identified
    let mut vulnerability_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let target_nodes = extract_targets_from_node(
        vec![Target::Multiply, Target::AssignDivide],
        source_unit.into(),
    );

    //For each target node that was extracted, check for the vulnerability patterns
    for node in target_nodes {
        //We can use unwrap because Target::Multiply is an expression
        let expression = node.expression().unwrap();

        match expression {
            pt::Expression::Multiply(loc, box_expression, _) => {
                let mut curr_expression = *box_expression;
                loop {
                    match curr_expression {
                        pt::Expression::Divide(_, _, _) => {
                            //Found case where division occurs before multiplication
                            vulnerability_locations.insert(loc);
                            break;
                        }
                        pt::Expression::Multiply(_, next_expression, _)
                        | pt::Expression::Parenthesis(_, next_expression) => {
                            //Continue to check the next expression for division
                            curr_expression = *next_expression;
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
            pt::Expression::AssignDivide(loc, _, box_expression) => {
                let mut curr_expression = *box_expression;
                loop {
                    match curr_expression {
                        pt::Expression::Multiply(_, _, _) => {
                            //Found case where multiplication occurs before division
                            vulnerability_locations.insert(loc);
                            break;
                        }
                        pt::Expression::Divide(_, next_expression, _)
                        | pt::Expression::Add(_, next_expression, _)
                        | pt::Expression::Subtract(_, next_expression, _)
                        | pt::Expression::Modulo(_, next_expression, _)
                        | pt::Expression::BitwiseAnd(_, next_expression, _)
                        | pt::Expression::BitwiseOr(_, next_expression, _)
                        | pt::Expression::BitwiseXor(_, next_expression, _)
                        | pt::Expression::ShiftLeft(_, next_expression, _)
                        | pt::Expression::ShiftRight(_, next_expression, _)
                        | pt::Expression::Parenthesis(_, next_expression) => {
                            //Continue to check the next expression for multiplication
                            curr_expression = *next_expression;
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    //Return the identified vulnerability locations
    vulnerability_locations
}

#[test]
fn test_divide_before_multiply_vulnerability() {
    let file_contents = r#"

    contract Contract0 {

        function arithmetic_operations() public {
            1 / 2 * 3; // Unsafe
            1 * 2 / 3; // Safe
            (1 / 2) * 3; // Unsafe
            (1 * 2) / 3; // Safe
            (1 / 2 * 3) * 4; // Unsafe (x2)
            (1 * 2 / 3) * 4; // Unsafe
            (1 / 2 / 3) * 4; // Unsafe
            1 / (2 + 3) * 4; // Unsafe
            (1 / 2 + 3) * 4; // Safe
            (1 / 2 - 3) * 4; // Safe
            (1 + 2 / 3) * 4; // Safe
            (1 / 2 - 3) * 4; // Safe
            (1 / 2 % 3) * 4; // Safe
            (1 / 2 | 3) * 4; // Safe
            (1 / 2 & 3) * 4; // Safe
            (1 / 2 ^ 3) * 4; // Safe
            (1 / 2 << 3) * 4; // Safe
            (1 / 2 >> 3) * 4; // Safe
            1 / (2 * 3 + 3); // Safe
            1 / ((2 / 3) * 3); // Unsafe
            1 / ((2 * 3) + 3); // Safe

            uint256 x = 5;
            x /= 2 * 3; // Unsafe
            x /= 2 / 3; // Safe
            x /= (2 * 3); // Unsafe
            x /= (1 / 2) * 3; // Unsafe (x2)
            x /= (1 * 2) * 3; // Unsafe
            x /= (2 * 3) / 4; // Unsafe
            x /= 2 * 3 / 4; // Unsafe
            x /= 2 * 3 - 4; // Unsafe
            x /= 2 * 3 % 4; // Unsafe
            x /= 2 * 3 | 4; // Unsafe
            x /= 2 * 3 & 4; // Unsafe
            x /= 2 * 3 ^ 4; // Unsafe
            x /= 2 * 3 << 4; // Unsafe
            x /= 2 * 3 >> 4; // Unsafe
            x /= 3 / 4; // Safe
            x /= 3 - 4; // Safe
            x /= 3 % 4; // Safe
            x /= 3 | 4; // Safe
            x /= 3 & 4; // Safe
            x /= 3 ^ 4; // Safe
            x /= 3 << 4; // Safe
            x /= 3 >> 4; // Safe
        }

    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let vulnerability_locations = divide_before_multiply_vulnerability(source_unit);
    assert_eq!(vulnerability_locations.len(), 22)
}
