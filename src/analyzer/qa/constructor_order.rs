use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};

// Constructor must be placed before any other function
pub fn constructor_order_qa(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each qa target identified
    let mut qa_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let target_nodes =
        ast::extract_target_from_node(Target::FunctionDefinition, source_unit.into());

    let mut fn_counter: u8 = 0; // up to 256 function definitions before reaching the constructor function

    //For each target node that was extracted, check for the qa patterns
    for _node in target_nodes {
        let contract_part = _node.contract_part().unwrap();

        if let pt::ContractPart::FunctionDefinition(box_fn_definition) = contract_part {
            match box_fn_definition.ty {
                pt::FunctionTy::Constructor => {
                    if fn_counter > 0 {
                        qa_locations.insert(box_fn_definition.loc);
                        break;
                    }
                }
                // Modifiers must be placed before constructor
                pt::FunctionTy::Modifier => continue,
                _ => {
                    fn_counter += 1;
                }
            }
        }
    }

    //Return the identified qa locations
    qa_locations
}

#[test]
fn test_constructor_order_qa() {
    let test_contracts = vec![
        r#"
    contract Contract0 {
        address public owner;
        function test() public {
            owner = address(0);
        }
        constructor() {
            owner = address(1);
        }
    }
    "#,
        r#"
    contract Contract0 {
        address public owner;
        receive() external payable {}
        constructor() {
            owner = address(1);
        }
    }
    "#,
        r#"
    contract Contract0 {
        address public owner;
        modifier onlyOwner {
            require(
            msg.sender == owner,
            "Only owner can call this function."
            );
            _;
        }
        constructor() {
            owner = address(1);
        }
    }
    "#,
        r#"
    contract Contract0 {
        address public owner;
        function test() public {
            owner = address(0);
        }
    }
    "#,
    ];

    let assertions = vec![1, 1, 0, 0];

    assert_eq!(test_contracts.len(), assertions.len());

    if assertions.len() > 0 {
        for i in 0..assertions.len() - 1 {
            let source_unit = solang_parser::parse(test_contracts[i], 0).unwrap().0;

            let qa_locations = constructor_order_qa(source_unit);
            assert_eq!(qa_locations.len(), assertions[i]);
        }
    }
}
