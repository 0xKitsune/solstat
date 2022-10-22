use solang_parser::pt::{self, Expression, Loc};
use solang_parser::{self, pt::SourceUnit};
use std::collections::HashSet;

use crate::analyzer::ast::Node;
use crate::analyzer::ast::{self, Target};

pub fn increment_decrement_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    //Get all increment/decrement expressions in unchecked blocks so that the analyzer does not mark these as optimization targets
    let block_nodes = ast::extract_target_from_node(Target::Block, source_unit.clone().into());
    let mut unchecked_locations: HashSet<Loc> = HashSet::new();
    for node in block_nodes {
        if let pt::Statement::Block {
            loc: _,
            unchecked,
            statements,
        } = node.statement().unwrap()
        {
            if unchecked {
                for statement in statements {
                    unchecked_locations
                        .extend(extract_pre_increment_pre_decrement(statement.into()));
                }
            }
        }
    }

    //Get all increment / decrement locations
    let locations = extract_increment_decrement(source_unit.into());

    for loc in locations {
        if !unchecked_locations.contains(&loc) {
            optimization_locations.insert(loc);
        }
    }

    optimization_locations
}

pub fn extract_increment_decrement(node: Node) -> HashSet<Loc> {
    let mut locations: HashSet<Loc> = HashSet::new();

    let target_nodes = ast::extract_targets_from_node(
        vec![
            Target::PreIncrement,
            Target::PreDecrement,
            Target::PostIncrement,
            Target::PostDecrement,
        ],
        node,
    );

    for node in target_nodes {
        //We can use expect because all targets are expressions
        let expression = node.expression().unwrap();

        match expression {
            Expression::PreIncrement(loc, _) => {
                locations.insert(loc);
            }
            Expression::PreDecrement(loc, _) => {
                locations.insert(loc);
            }
            Expression::PostIncrement(loc, _) => {
                locations.insert(loc);
            }
            Expression::PostDecrement(loc, _) => {
                locations.insert(loc);
            }

            _ => {}
        }
    }
    locations
}

pub fn extract_pre_increment_pre_decrement(node: Node) -> HashSet<Loc> {
    let mut locations: HashSet<Loc> = HashSet::new();

    let target_nodes =
        ast::extract_targets_from_node(vec![Target::PreIncrement, Target::PreDecrement], node);

    for node in target_nodes {
        //We can use expect because all targets are expressions
        let expression = node.expression().unwrap();

        match expression {
            Expression::PreIncrement(loc, _) => {
                locations.insert(loc);
            }
            Expression::PreDecrement(loc, _) => {
                locations.insert(loc);
            }

            _ => {}
        }
    }
    locations
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
