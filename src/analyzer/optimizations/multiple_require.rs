use std::collections::HashSet;

use solang_parser::pt::{Expression, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};

//Use multiple require statements instead of one single require statement with multiple conditions
pub fn multiple_require_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let target_nodes = ast::extract_target_from_node(Target::FunctionCall, source_unit.into());

    for node in target_nodes {
        //We can use unwrap because Target::FunctionCall is an expression
        let expression = node.expression().unwrap();

        if let Expression::FunctionCall(loc, function_identifier, function_call_expressions) =
            expression
        {
            //if the function call identifier is a variable
            if let Expression::Variable(identifier) = *function_identifier {
                //if the identifier name is "require"
                if identifier.name == "require".to_string() {
                    //for each expression in the function call expressions
                    for func_call_expression in function_call_expressions {
                        //if there is an and expression (ie. &&)
                        if let Expression::And(_, _, _) = func_call_expression {
                            //add the location to the list of optimization locations
                            optimization_locations.insert(loc);
                            continue;
                        }
                    }
                }
            }
        }
    }

    optimization_locations
}

#[test]
fn test_multiple_require_optimization() {
    let file_contents = r#"
    contract Contract0 {
        function addressInternalBalance() public returns (uint256) {

            uint256 a = 100;
            uint256 b = 100;
            uint256 c = 100;

            require(true, "some message");

            require(true && a==b, "some message");
            require(true && a==b && b==c, "thing");

            return address(this).balance;


        }
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = multiple_require_optimization(source_unit);

    assert_eq!(optimization_locations.len(), 2)
}
