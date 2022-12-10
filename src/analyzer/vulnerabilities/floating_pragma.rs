use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{extract_target_from_node, Target};

pub fn floating_pragma_vulnerability(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each vulnerability target identified
    let mut vulnerability_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let target_nodes = extract_target_from_node(Target::PragmaDirective, source_unit.into());

    //For each target node that was extracted, check for the vulnerability patterns
    for node in target_nodes {
        //We can use unwrap because Target::PragmaDirective is a source unit part
        let source_unit_part = node.source_unit_part().unwrap();

        if let pt::SourceUnitPart::PragmaDirective(loc, _, pragma) = source_unit_part {
            if pragma.string.contains('^') {
                vulnerability_locations.insert(loc);
            }
        }
    }

    //Return the identified vulnerability locations
    vulnerability_locations
}

#[test]
fn test_floating_pragma_vulnerability() {
    let file_contents = r#"

    pragma solidity ^0.8.16;

    contract Contract0 {

    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let vulnerability_locations = floating_pragma_vulnerability(source_unit);
    assert_eq!(vulnerability_locations.len(), 1)
}
