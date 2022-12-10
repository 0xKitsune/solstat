use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{extract_target_from_node, Target};

pub fn divide_before_multiply_vulnerability(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each vulnerability target identified
    let mut vulnerability_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let target_nodes = extract_target_from_node(Target::Multiply, source_unit.into());

    //For each target node that was extracted, check for the vulnerability patterns
    for node in target_nodes {
        //We can use unwrap because Target::Multiply is an expression
        let expression = node.expression().unwrap();

        if let pt::Expression::Multiply(loc, box_expression, _) = expression {
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
            1 / (2 * 3 + 3); // Safe
            1 / ((2 / 3) * 3); // Unsafe
            1 / ((2 * 3) + 3); // Safe
        }

    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let vulnerability_locations = divide_before_multiply_vulnerability(source_unit);
    assert_eq!(vulnerability_locations.len(), 8)
}
