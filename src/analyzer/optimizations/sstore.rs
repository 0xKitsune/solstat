use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};
use crate::analyzer::utils;

pub fn sstore_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    //Get all storage variables
    let storage_variables = utils::get_32_byte_storage_variables(source_unit.clone(), true, true);

    //Extract the target nodes from the source_unit
    let target_nodes = ast::extract_target_from_node(Target::Assign, source_unit.into());

    for node in target_nodes {
        //We can use unwrap because Target::Assign is an expression
        let expression = node.expression().unwrap();

        //if the expression is an Assign
        if let pt::Expression::Assign(loc, box_expression, _) = expression {
            //if the first expr in the assign expr is a variable
            if let pt::Expression::Variable(identifier) = *box_expression {
                //if the variable name exists in the storage variable hashmap
                if storage_variables.contains_key(&identifier.name) {
                    //add the location to the optimization locations
                    optimization_locations.insert(loc);
                }
            }
        }
    }
    //Return the identified optimization locations
    optimization_locations
}
#[test]
fn test_sstore_optimization() {
    let file_contents = r#"
    
    pragma solidity >= 0.8.0;
    contract Contract {
       
        uint256 thing = 100;
        address someAddress = address(0);
        bytes someBytes;
        
    
       
        function testFunction() public {
             thing = 1+2;
             someAddress = msg.sender;
             someBytes = bytes(0);


        }
    }
 
    "#;
    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = sstore_optimization(source_unit);

    assert_eq!(optimization_locations.len(), 3);
}
