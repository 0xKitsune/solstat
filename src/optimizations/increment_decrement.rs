use solang_parser::pt::{Expression, Loc};
use solang_parser::{self, pt::SourceUnit};
use std::collections::HashSet;

use crate::ast::ast::{self, Target};

pub fn increment_decrement_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let target_nodes = ast::extract_targets_from_node(
        vec![
            Target::PreIncrement,
            Target::PreDecrement,
            Target::PostIncrement,
            Target::PostDecrement,
        ],
        source_unit.into(),
    );

    for node in target_nodes {
        //We can use expect because all targets are expressions
        let expression = node.expression().unwrap();

        match expression {
            Expression::PreIncrement(loc, _) => {
                optimization_locations.insert(loc);
            }
            Expression::PreDecrement(loc, _) => {
                optimization_locations.insert(loc);
            }
            Expression::PostIncrement(loc, _) => {
                optimization_locations.insert(loc);
            }
            Expression::PostDecrement(loc, _) => {
                optimization_locations.insert(loc);
            }

            _ => {}
        }
    }
    optimization_locations
}

#[test]
fn test_increment_optimization() {
    let file_contents = r#"
  
    contract Contract0 {
        function iPlusPlus(){
            uint256  i = 0;
            i++;
        }
    
        function plusPlusI() public {
            uint256  i = 0;
            ++i;
    
        for (uint256 j = 0; j < numNfts; ) {
                bytes32 hash = keccak256(abi.encode(ORDER_ITEM_HASH, nfts[i].collection, _tokensHash(nfts[i].tokens)));
                hashes[i] = hash;
                unchecked {
                  ++j;
                }
              }
    
            unchecked{
                i++;
            }
        }
        
    }
    
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = increment_decrement_optimization(source_unit);

    assert_eq!(optimization_locations.len(), 3)
}
