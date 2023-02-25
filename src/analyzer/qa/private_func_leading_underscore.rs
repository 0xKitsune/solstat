use std::collections::HashSet;

use solang_parser::pt::{self, FunctionTy, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};

pub fn private_func_leading_underscore(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each qa target identified
    let mut qa_locations: HashSet<Loc> = HashSet::new();
    //Extract the target nodes from the source_unit
    let target_nodes =
        ast::extract_target_from_node(Target::FunctionDefinition, source_unit.into());

    for node in target_nodes {
        let contract_part = node.contract_part().unwrap();

        if let pt::ContractPart::FunctionDefinition(box_fn_definition) = contract_part {
            if FunctionTy::Function != box_fn_definition.ty {
                continue;
            }

            for attr in box_fn_definition.attributes {
                if let pt::FunctionAttribute::Visibility(v) = attr {
                    let fn_def_data = box_fn_definition.name.clone();
                    if fn_def_data.is_some() {
                        let fn_data = fn_def_data.unwrap();
                        match v {
                            pt::Visibility::Public(_) | pt::Visibility::External(_) => {
                                if fn_data.name.starts_with('_') {
                                    qa_locations.insert(fn_data.loc);
                                }
                            }
                            // Private or Internal functions
                            _ => {
                                if !fn_data.name.starts_with('_') {
                                    qa_locations.insert(fn_data.loc);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    //Return the identified qa locations
    qa_locations
}

#[test]
fn test_private_func_leading_underscore() {
    let file_contents = r#"
    
    contract Contract0 {
        
        function msgSender() internal view returns(address) {
            return msg.sender;
        }

        function _msgSender() internal view returns(address) {
            return msg.sender;
        }

        function _msgData() private view returns(bytes calldata) {
            return msg.data;
        }

        function msgData() private view returns(bytes calldata) {
            return msg.data;
        }
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let qa_locations = private_func_leading_underscore(source_unit);
    assert_eq!(qa_locations.len(), 2)
}
