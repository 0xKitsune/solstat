use std::collections::HashSet;

use solang_parser::pt::{self, Loc, SourceUnit};

use crate::analyzer::ast::{self, Target};

pub fn address_zero_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let target_nodes =
        ast::extract_targets_from_node(vec![Target::Equal, Target::NotEqual], source_unit.into());

    for node in target_nodes {
        //We can use unwrap because Target::Equal and Target::NotEqual are expressions

        let expression = node.expression().unwrap();

        match expression {
            pt::Expression::NotEqual(loc, box_expression, box_expression_1) => {
                if check_for_address_zero(box_expression)
                    || check_for_address_zero(box_expression_1)
                {
                    optimization_locations.insert(loc);
                }
            }
            pt::Expression::Equal(loc, box_expression, box_expression_1) => {
                if check_for_address_zero(box_expression)
                    || check_for_address_zero(box_expression_1)
                {
                    optimization_locations.insert(loc);
                }
            }
            _ => {}
        }
    }

    optimization_locations
}

fn check_for_address_zero(box_expression: Box<pt::Expression>) -> bool {
    //create a boolean to determine if address(0) is present
    let mut address_zero: bool = false;

    //if the first expression is address(0)
    if let pt::Expression::FunctionCall(_, func_call_box_expression, vec_expression) =
        *box_expression
    {
        if let pt::Expression::Type(_, ty) = *func_call_box_expression {
            if let pt::Type::Address = ty {
                if let pt::Expression::NumberLiteral(_, val, _) = &vec_expression[0] {
                    if val == "0" {
                        address_zero = true;
                    }
                }
            }
        }
    }

    //return true or false for address_zero
    address_zero
}

#[test]
fn test_address_zero_optimization() {
    let file_contents = r#"
    
    contract Contract0 {

        function ownerNotZero(address _addr) public pure {
            require(_addr == address(0), "zero address");
        }

        function ownerNotZero(address _addr) public pure {
            require(_addr != address(0), "zero address");
        }

        function ownerNotZero1(address _addr) public pure {
            require(address(0) == _addr, "zero address");
        }

        function ownerNotZero1(address _addr) public pure {
            require(address(0) != _addr, "zero address");
        }

     }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = address_zero_optimization(source_unit);

    assert_eq!(optimization_locations.len(), 4)
}
