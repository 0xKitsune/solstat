use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};

//Use selfbalance() instead of address(this).balance()
pub fn address_balance_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let target_nodes = ast::extract_target_from_node(Target::MemberAccess, source_unit.into());

    for node in target_nodes {
        //We can use unwrap because Target::MemberAccess is an expression
        let expression = node.expression().unwrap();

        if let pt::Expression::MemberAccess(loc, box_expression, identifier) = expression {
            if let pt::Expression::FunctionCall(_, box_expression, _) = *box_expression {
                if let pt::Expression::Type(_, ty) = *box_expression {
                    if let pt::Type::Address = ty {
                        //if address(0x...).balance or address(this).balance
                        if identifier.name == "balance".to_string() {
                            optimization_locations.insert(loc);
                        }
                    }
                }
            }
        }
    }

    optimization_locations
}

#[test]
fn test_address_balance_optimization() {
    let file_contents = r#"
    
contract Contract0 {
    function addressInternalBalance(){
        uint256 bal = address(this).balance;
        bal++;
    }

    function addressExternalBalance(address addr) public {
        uint256 bal = address(addr).balance;
        bal++;
    }
}

    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = address_balance_optimization(source_unit);

    assert_eq!(optimization_locations.len(), 2)
}
