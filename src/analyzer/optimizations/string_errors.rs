use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};
use crate::analyzer::utils;

pub fn string_error_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let solidity_version = utils::get_solidity_version_from_source_unit(source_unit.clone())
        .expect("Could not extract Solidity version from source unit.");

    if solidity_version.1 >= 8 && solidity_version.2 >= 4 {
        //Extract the target nodes from the source_unit
        let target_nodes = ast::extract_target_from_node(Target::FunctionCall, source_unit.into());

        for node in target_nodes {
            //We can use unwrap because Target::FunctionCall is an expression
            let expression = node.expression().unwrap();

            if let pt::Expression::FunctionCall(_, function_identifier, func_call_expressions) =
                expression
            {
                //if the function call identifier is a variable
                if let pt::Expression::Variable(identifier) = *function_identifier {
                    //if the identifier name is "require"
                    if identifier.name == "require".to_string() {
                        //If the require statement contains strings
                        if let pt::Expression::StringLiteral(vec_string_literal) =
                            func_call_expressions[func_call_expressions.len() - 1].clone()
                        {
                            optimization_locations.insert(vec_string_literal[0].loc);
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
fn test_string_error_optimization() {
    //test when base solidiy version is > than 0.8.4
    let file_contents = r#"
     pragma solidity >=0.8.13;
 
     contract Contract0 {
         function addressInternalBalance() public returns (uint256) {
 
             require(true, "some message");
 
             require(true && a==b, "some message");
             require(true && a==b && b==c, "thing");
 
             return address(this).balance;
         }
     }
     "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = string_error_optimization(source_unit);

    assert_eq!(optimization_locations.len(), 3);

    //test when base solidiy version is < than 0.8.4
    let file_contents_1 = r#"
     pragma solidity <= 0.8.3;
 
     contract Contract0 {
         function addressInternalBalance() public returns (uint256) {
 
             require(true, "some message");
 
             require(true && a==b, "some message");
             require(true && a==b && b==c, "thing");
 
             return address(this).balance;
         }
     }
     "#;

    let source_unit_1 = solang_parser::parse(file_contents_1, 0).unwrap().0;

    let optimization_locations_1 = string_error_optimization(source_unit_1);

    assert_eq!(optimization_locations_1.len(), 0);
}
