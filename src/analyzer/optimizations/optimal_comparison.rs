use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};

pub fn optimal_comparison_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let target_nodes = ast::extract_targets_from_node(
        vec![Target::MoreEqual, Target::LessEqual],
        source_unit.into(),
    );

    //For each target node that was extracted, check for the optimization patterns
    for node in target_nodes {
        //We can use unwrap because Target::MemberAccess is an expression
        let expression = node.expression().unwrap();

        match expression {
            // >= operator
            pt::Expression::MoreEqual(loc, _box_expression_0, _box_expression_1) => {
                println!("runs");
                optimization_locations.insert(loc);
            }

            // <= operator
            pt::Expression::LessEqual(loc, _box_expression_0, _box_expression_1) => {
                optimization_locations.insert(loc);
            }

            _ => {}
        }
    }

    //Return the identified optimization locations
    optimization_locations
}

#[test]
fn test_optimal_comparison_optimization() {
    let file_contents = r#"
    
contract Contract0 {
    function greaterThanOrEqualTo(uint256 a, uint256 b) public pure {
        return a >= b;
    }

    function lessThanOrEqualTo(uint256 a, uint256 b) public pure {
        return a <= b;
    }
}
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = optimal_comparison_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 2)
}
