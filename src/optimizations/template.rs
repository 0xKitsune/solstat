use std::collections::HashSet;

use solang_parser::pt::Loc;
use solang_parser::{self, pt::SourceUnit};

use crate::ast::ast::{self, Target};

pub fn _template_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    let optimization_locations: HashSet<Loc> = HashSet::new();

    let target_nodes = ast::extract_target_from_node(Target::None, source_unit.into());

    for _node in target_nodes {}

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
