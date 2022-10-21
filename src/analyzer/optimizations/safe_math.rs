use solang_parser::pt::{self, Expression, Loc, SourceUnit, SourceUnitPart};
use std::collections::HashSet;

use crate::analyzer::{
    ast::{self, Node, Target},
    utils,
};

pub fn safe_math_pre_080_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    safe_math_optimization(source_unit, true)
}

pub fn safe_math_post_080_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    safe_math_optimization(source_unit, false)
}

pub fn safe_math_optimization(source_unit: SourceUnit, pre_080: bool) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let solidity_version = utils::get_solidity_version_from_source_unit(source_unit.clone())
        .expect("Could not extract solidity version from source unit");

    if (pre_080 && solidity_version.1 < 8) || (!pre_080 && solidity_version.1 >= 8) {
        //if using safe math
        if check_if_using_safe_math(source_unit.clone()) {
            //get all locations that safe math functions are used
            optimization_locations.extend(parse_contract_for_safe_math_functions(source_unit));
        }
    }

    optimization_locations
}

fn check_if_using_safe_math(source_unit: SourceUnit) -> bool {
    let mut using_safe_math: bool = false;

    let target_nodes = ast::extract_target_from_node(Target::Using, source_unit.into());

    for node in target_nodes {
        match node {
            Node::SourceUnitPart(source_unit_part) => {
                if let pt::SourceUnitPart::Using(box_using) = source_unit_part {
                    if let pt::UsingList::Library(identifier_path) = box_using.list {
                        for identifier in identifier_path.identifiers {
                            if identifier.name == "SafeMath" {
                                using_safe_math = true;
                            }
                        }
                    }
                }
            }

            Node::ContractPart(contract_part) => {
                if let pt::ContractPart::Using(box_using) = contract_part {
                    if let pt::UsingList::Library(identifier_path) = box_using.list {
                        for identifier in identifier_path.identifiers {
                            if identifier.name == "SafeMath" {
                                using_safe_math = true;
                            }
                        }
                    }
                }
            }

            _ => {}
        }
    }

    using_safe_math
}

fn parse_contract_for_safe_math_functions(source_unit: SourceUnit) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let target_nodes = ast::extract_target_from_node(Target::FunctionCall, source_unit.into());

    for node in target_nodes {
        //Can use unwrap because Target::FunctionCall.expression() will always be Some(expression)
        let expression = node.expression().unwrap();

        //if the expression is a function call
        if let Expression::FunctionCall(_, function_identifier, _) = expression {
            //if the function call identifier is a variable
            if let Expression::MemberAccess(loc, _, identifier) = *function_identifier {
                //if the identifier name is add, sub, mul or div
                if identifier.name == "add".to_string() {
                    optimization_locations.insert(loc);
                } else if identifier.name == "sub".to_string() {
                    optimization_locations.insert(loc);
                } else if identifier.name == "mul".to_string() {
                    optimization_locations.insert(loc);
                } else if identifier.name == "div".to_string() {
                    optimization_locations.insert(loc);
                }
            }
        }
    }

    optimization_locations
}

#[test]
fn test_analyze_for_safe_math_pre_080() {
    let file_contents = r#"
    
    pragma solidity >= 0.7.0;
    contract Contract {
        /// *** Libraries ***
        using SafeMath for uint256;
        using SafeMath for uint16;
    
       
        function testFunction(){
            uint256 something = 190092340923434;
            uint256 somethingElse = 1;

            uint256 thing = something.add(somethingElse);
            uint256 thing1 = something.sub(somethingElse);
            uint256 thing2 = something.mul(somethingElse);
            uint256 thing3 = something.div(somethingElse);

        }
    }
 
    "#;
    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = safe_math_pre_080_optimization(source_unit);

    assert_eq!(optimization_locations.len(), 4);
}

#[test]
fn test_analyze_for_safe_math_post_080() {
    let file_contents = r#"
    
    pragma solidity >= 0.8.13;
    contract Contract {
        /// *** Libraries ***
        using SafeMath for uint256;
        using SafeMath for uint16;
    
       
        function testFunction(){
            uint256 something = 190092340923434;
            uint256 somethingElse = 1;

            uint256 thing = something.add(somethingElse);
            uint256 thing1 = something.sub(somethingElse);
            uint256 thing2 = something.mul(somethingElse);
            uint256 thing3 = something.div(somethingElse);

        }
    }
 
    "#;
    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = safe_math_post_080_optimization(source_unit);

    assert_eq!(optimization_locations.len(), 4);
}
