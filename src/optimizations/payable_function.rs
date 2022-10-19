use std::collections::HashSet;

use solang_parser::pt::{Expression, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::ast::ast::{self, Target};

pub fn payable_function_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let target_nodes =
        ast::extract_target_from_node(Target::FunctionDefinition, source_unit.into());

    for node in target_nodes {
        //We can use expect because Target::FunctionDefinition is an expression
        let contract_part = node.contract_part().unwrap();

        match contract_part {
            _ => {}
        }
    }

    optimization_locations
}

#[test]
fn test_analyze_for_payable_function_optimization() {
    let file_contents = r#"
    

    contract Contract0 {

        function div2(uint256 a, uint256 b) public pure {
            
        }

        function mul2(uint256 a, uint256 b) external view {
            
        }
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = multiple_require_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 2)
}
