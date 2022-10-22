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
    let mut is_even: bool = false;

    //if the first expression is a number literal that is a power of 2
    if let Expression::NumberLiteral(_, val_string, _) = *box_expression {
        let value = val_string
            .parse::<u32>()
            .expect("Could not parse NumberLiteral value from string to u32");

        if (value != 0) && ((value & (value - 1)) == 0) {
            is_even = true;
        }
    }

    //if the first expression is a number literal that is a power of 2
    if let Expression::NumberLiteral(_, val_string, _) = *box_expression_1 {
        let value = val_string
            .parse::<u32>()
            .expect("Could not parse NumberLiteral value from string to u32");

        if (value != 0) && ((value & (value - 1)) == 0) {
            is_even = true;
        }
    }

    is_even
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
        }
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = shift_math_optimization(source_unit);

    assert_eq!(optimization_locations.len(), 3)
}
