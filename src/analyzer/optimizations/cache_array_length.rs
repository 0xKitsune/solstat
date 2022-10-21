use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};

pub fn cache_array_length_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let target_nodes = ast::extract_target_from_node(Target::For, source_unit.into());

    //For each target node that was extracted, check for the optimization patterns
    for node in target_nodes {
        //Can unwrap because Target::For will always be a statement
        let statement = node.statement().unwrap();

        if let pt::Statement::For(_, _, option_box_expression, _, _) = statement {
            //get all of the .length in the for loop definition
            if option_box_expression.is_some() {
                let box_expression = option_box_expression.unwrap();

                let member_access_nodes =
                    ast::extract_target_from_node(Target::MemberAccess, box_expression.into());

                for node in member_access_nodes {
                    //Can unwrap because Target::MemberAccess will always be an expression
                    let member_access = node.expression().unwrap();
                    if let pt::Expression::MemberAccess(loc, _, identifier) = member_access {
                        if identifier.name == "length" {
                            optimization_locations.insert(loc);
                        }
                    }
                }
            }
        }
    }

    //Return the identified optimization locations
    optimization_locations
}

#[test]
fn test_cache_array_length_optimization() {
    let file_contents = r#"

    contract Contract1 {


        //loop with i++
        function memoryArray(uint256[] memory arr) public {
            uint256 j;
            for (uint256 i; i < arr.length; i++) {
                j = arr[i] + 10;
            }
        }
    
        //loop with i++
        function calldataArray(uint256[] calldata arr) public {
            uint256 j;
            for (uint256 i; i < 100; i++) {
                j = arr[i] + arr.length;
            }
        }
    
        //loop with i++
        function memoryArray(uint256[] memory arr) public {
            uint256 j;
            for (uint256 i;  arr.length<1000; i++) {
                arr[i] = 10;
            }
        }
    
        }    
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = cache_array_length_optimization(source_unit);

    assert_eq!(optimization_locations.len(), 2)
}
