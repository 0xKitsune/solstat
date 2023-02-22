use std::collections::HashSet;
use std::u32;

use solang_parser::pt::{Expression, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};

pub fn shift_math_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let target_nodes =
        ast::extract_targets_from_node(vec![Target::Multiply, Target::Divide], source_unit.into());

    for node in target_nodes {
        //We can use expect because both Target::Multiply and Target::Divide are expressions
        let expression = node.expression().expect("Node is not an expression");

        match expression {
            Expression::Multiply(loc, box_expression, box_expression_1) => {
                if check_if_inputs_are_power_of_two(box_expression, box_expression_1) {
                    optimization_locations.insert(loc);
                }
            }

            Expression::Divide(loc, box_expression, box_expression_1) => {
                if check_if_inputs_are_power_of_two(box_expression, box_expression_1) {
                    optimization_locations.insert(loc);
                }
            }

            _ => {}
        }
    }
    optimization_locations
}

fn check_if_inputs_are_power_of_two(
    box_expression: Box<Expression>,
    box_expression_1: Box<Expression>,
) -> bool {
    //create a boolean to determine if either of the inputs are a power of two

    //if the first expression is a number literal that is a power of 2
    if let Expression::NumberLiteral(_, val_string, _) = *box_expression {
        match val_string.parse::<u128>() {
            Ok(value) => {
                if (value != 0) && ((value & (value - 1)) == 0) {
                    return true;
                }
            }
            Err(_) => println!(
                "Could not parse NumberLiteral value '{}' from string to u128",
                val_string
            ),
        }
    }

    //if the first expression is a number literal that is a power of 2
    if let Expression::NumberLiteral(_, val_string, _) = *box_expression_1 {
        match val_string.parse::<u128>() {
            Ok(value) => {
                if (value != 0) && ((value & (value - 1)) == 0) {
                    return true;
                }
            }
            Err(_) => println!(
                "Could not parse NumberLiteral value '{}' from string to u128",
                val_string
            ),
        }
    }

    return false;
}

#[test]
fn test_shift_math_optimization() {
    let file_contents = r#"

    contract Contract0 {

        function mul2(uint256 a, uint256 b) public pure {
            uint256 a = 10 * 2;

            uint256 b = 2 * a;
            uint256 c = a * b;

            uint256 d = (a * b) * 2;
            uint256 q = 340282366920938463463374607431768211455 * 2;
            uint256 p = 340282366920938463463374607431768211456 * 3;

        }
    }
    "#;
    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = shift_math_optimization(source_unit);

    assert_eq!(optimization_locations.len(), 4)
}
