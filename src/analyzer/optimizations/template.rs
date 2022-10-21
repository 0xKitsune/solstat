use std::collections::HashSet;

use solang_parser::pt::Loc;
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};

pub fn _template_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each optimization target identified
    let optimization_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let target_nodes = ast::extract_target_from_node(Target::None, source_unit.into());
    //If searching for multiple target nodes, use the following function instead and pass a vec of Target
    // let target_nodes = ast::extract_targets_from_node(vec![Target::Target1, Target::Target2], source_unit.into());

    //For each target node that was extracted, check for the optimization patterns
    for _node in target_nodes {}

    //Return the identified optimization locations
    optimization_locations
}

#[test]
fn test_template_optimization() {
    let file_contents = r#"
    
    contract Contract0 {

    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = _template_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 0)
}
