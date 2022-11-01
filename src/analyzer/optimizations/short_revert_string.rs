use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::{
    ast::{self, Target},
    utils,
};

pub fn short_revert_string_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations = HashSet::<Loc>::new();

    let solidity_version = utils::get_solidity_version_from_source_unit(source_unit.clone())
        .expect("Could not extract Solidity version from source unit.");

    if !(solidity_version.1 >= 8 && solidity_version.2 >= 4) {
        let target_nodes = ast::extract_target_from_node(Target::FunctionCall, source_unit.into());

        for node in target_nodes {
            let expression = node.expression().unwrap();

            if let pt::Expression::FunctionCall(_, ident, expressions) = expression {
                match (*ident, expressions.last()) {
                    (
                        // identifier is variable
                        pt::Expression::Variable(identifier),
                        // last expression is string literal
                        Some(pt::Expression::StringLiteral(literals)),
                    ) if identifier.name.eq("require") => {
                        if let Some(literal) = literals.get(0) {
                            if literal.string.len() >= 32 {
                                optimization_locations.insert(literal.loc);
                            }
                        }
                    }
                    _ => (),
                };
            }
        }
    }

    //Return the identified optimization locations
    optimization_locations
}

#[test]
fn test_short_revert_string() {
    let file_contents = r#"
    pragma solidity 0.8.0;
    
    contract Contract0 {
        function expensiveRevertStrings() {
            require(a < b, "long revert string over 32 bytes");
        }

        function cheapRevertStrings() {
            require(a < b, "a");
        }

        function noRevertMessage() {
            require(a < b);
        }
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = short_revert_string_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 1);

    let invalid_version_content = r#"
    pragma solidity 0.8.14;
    
    contract Contract0 {
        function expensiveRevertStrings() {
            require(a < b, "long revert string over 32 bytes");
        }
    }
    "#;

    let source_unit = solang_parser::parse(invalid_version_content, 0).unwrap().0;

    let optimization_locations = short_revert_string_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 0);
}
