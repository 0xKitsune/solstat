use crate::analyzer::ast::{self, extract_target_from_node, Target};
use regex::Regex;
use solang_parser::pt::{self, Loc, SourceUnit, SourceUnitPart};
use std::collections::HashMap;

pub type LineNumber = i32;

//Returns the size of the type in bytes
pub fn get_type_size(expression: pt::Expression) -> u16 {
    if let pt::Expression::Type(_, ty) = expression {
        match ty {
            pt::Type::Address => return 256,
            pt::Type::AddressPayable => return 256,
            pt::Type::Bytes(_size) => return (_size as u16) * 4,
            pt::Type::Bool => return 1,
            pt::Type::Int(_size) => return _size,
            pt::Type::Uint(_size) => return _size,
            _ => return 256,
        }
    }

    //TODO: add error handling that bubbles up if the expression passed in is not a type
    256
}

//get line number of start of character range
pub fn get_line_number(char_number: usize, file_contents: &str) -> i32 {
    let re = Regex::new(r"\n").unwrap();
    let mut i = 1;
    for capture in re.captures_iter(file_contents).into_iter() {
        for c in capture.iter() {
            if c.unwrap().start() > char_number {
                //+1 since line numbers start at 1
                return i;
            } else {
                i = i + 1;
            }
        }
    }

    return 0;
}

pub fn storage_slots_used(variables: Vec<u16>) -> u32 {
    //set a variable to keep track of how many bytes have been used in the slot
    let mut bytes_used_in_slot = 0;
    //--------------------- test slot usage of unordered variable sizes ---------------------------------------

    //loop through the unordered variable sizes and count the amount of slots used
    let mut slots_used = 0;
    for variable_size in variables {
        //if the next variable size
        if bytes_used_in_slot + variable_size > 256 {
            //add a slot used
            slots_used += 1;

            //update bytes used in slot
            bytes_used_in_slot = variable_size;
        } else {
            bytes_used_in_slot += variable_size;
        }
    }

    //if the bytes in slot is > 0 and the last variable has been accounted for, add one more slot used
    if bytes_used_in_slot > 0 {
        slots_used += 1;
    }

    slots_used
}

pub fn get_32_byte_storage_variables(
    source_unit: pt::SourceUnit,
    ignore_constants: bool,
    ignore_immutables: bool,
) -> HashMap<String, (Option<pt::VariableAttribute>, Loc)> {
    let mut storage_variables: HashMap<String, (Option<pt::VariableAttribute>, Loc)> =
        HashMap::new();

    let target_nodes = extract_target_from_node(Target::ContractDefinition, source_unit.into());

    for node in target_nodes {
        let source_unit_part = node.source_unit_part().unwrap();

        if let pt::SourceUnitPart::ContractDefinition(contract_definition) = source_unit_part {
            'outer: for part in contract_definition.parts {
                if let pt::ContractPart::VariableDefinition(box_variable_definition) = part {
                    let mut variable_attribute: Option<pt::VariableAttribute> = None;
                    //if the variable is constant, mark constant_variable as true
                    for attribute in box_variable_definition.attrs {
                        if let pt::VariableAttribute::Constant(variable_attribute_loc) = attribute {
                            if ignore_constants {
                                continue 'outer;
                            }
                            variable_attribute =
                                Some(pt::VariableAttribute::Constant(variable_attribute_loc));
                        } else if let pt::VariableAttribute::Immutable(variable_attribute_loc) =
                            attribute
                        {
                            if ignore_immutables {
                                continue 'outer;
                            }

                            variable_attribute =
                                Some(pt::VariableAttribute::Immutable(variable_attribute_loc));
                        }
                    }

                    if let pt::Expression::Type(loc, ty) = box_variable_definition.ty {
                        if let pt::Type::Mapping(_, _, _) = ty {
                        } else {
                            storage_variables.insert(
                                box_variable_definition.name.name,
                                (variable_attribute, loc),
                            );
                        }
                    }
                }
            }
        }
    }

    storage_variables
}

pub fn get_constant_variables(source_unit: pt::SourceUnit) -> HashMap<String, Loc> {
    let mut variables: HashMap<String, Loc> = HashMap::new();

    let target_nodes = extract_target_from_node(Target::ContractDefinition, source_unit.into());

    for node in target_nodes {
        let source_unit_part = node.source_unit_part().unwrap();

        if let pt::SourceUnitPart::ContractDefinition(contract_definition) = source_unit_part {
            for part in contract_definition.parts {
                if let pt::ContractPart::VariableDefinition(box_variable_definition) = part {
                    //if the variable is constant, mark constant_variable as true
                    for attribute in box_variable_definition.attrs {
                        if let pt::VariableAttribute::Constant(loc) = attribute {
                            variables.insert(box_variable_definition.name.to_string(), loc);
                        }
                    }
                }
            }
        }
    }

    variables
}

pub fn get_immutable_variables(source_unit: pt::SourceUnit) -> HashMap<String, Loc> {
    let mut variables: HashMap<String, Loc> = HashMap::new();

    let target_nodes = extract_target_from_node(Target::ContractDefinition, source_unit.into());

    for node in target_nodes {
        let source_unit_part = node.source_unit_part().unwrap();

        if let pt::SourceUnitPart::ContractDefinition(contract_definition) = source_unit_part {
            for part in contract_definition.parts {
                if let pt::ContractPart::VariableDefinition(box_variable_definition) = part {
                    //if the variable is constant, mark constant_variable as true
                    for attribute in box_variable_definition.attrs {
                        if let pt::VariableAttribute::Immutable(loc) = attribute {
                            variables.insert(box_variable_definition.name.to_string(), loc);
                        }
                    }
                }
            }
        }
    }

    variables
}

//Returns minor, major, patch version from a contract file
pub fn get_solidity_version_from_source_unit(source_unit: SourceUnit) -> Option<(i32, i32, i32)> {
    let target_nodes =
        ast::extract_target_from_node(Target::PragmaDirective, source_unit.clone().into());

    //check if the solidity version is < 0.8.0
    let mut solidity_minor_version: i32 = 0;
    for node in target_nodes {
        let source_unit_part = node.source_unit_part().unwrap();

        if let SourceUnitPart::PragmaDirective(_, _, solidity_version_literal) = source_unit_part {
            let minor_major_patch_version =
                get_solidity_major_minor_patch_version(&solidity_version_literal.string)
                    .iter()
                    .map(|f| f.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

            return Some((
                minor_major_patch_version[0],
                minor_major_patch_version[1],
                minor_major_patch_version[2],
            ));
        }
    }

    None
}

pub fn get_solidity_major_version(solidity_version_str: &str) -> i32 {
    let major_minor_patch_vec = get_solidity_major_minor_patch_version(solidity_version_str);
    major_minor_patch_vec[0].parse::<i32>().unwrap()
}
pub fn get_solidity_minor_version(solidity_version_str: &str) -> i32 {
    let major_minor_patch_vec = get_solidity_major_minor_patch_version(solidity_version_str);
    major_minor_patch_vec[1].parse::<i32>().unwrap()
}
pub fn get_solidity_patch_version(solidity_version_str: &str) -> i32 {
    let major_minor_patch_vec = get_solidity_major_minor_patch_version(solidity_version_str);
    major_minor_patch_vec[2].parse::<i32>().unwrap()
}

pub fn get_solidity_major_minor_patch_version(solidity_version_str: &str) -> Vec<&str> {
    //get the minor.patch version from the solidity semantic version
    let mut major_minor_patch_version_str = "0.0.0";
    let major_minor_patch_version_re = Regex::new(r"\d+\.\d+\.+\d+").unwrap();
    for capture in major_minor_patch_version_re
        .captures_iter(solidity_version_str)
        .into_iter()
    {
        for minor_version in capture.iter() {
            major_minor_patch_version_str = minor_version.unwrap().as_str();
        }
    }

    major_minor_patch_version_str
        .split(".")
        .collect::<Vec<&str>>()
}

#[test]
fn test_get_solidity_version() {
    let solidity_version_str = r#"pragma solidity >=0.8.13;"#;

    let major_version = get_solidity_major_version(solidity_version_str);
    let minor_version = get_solidity_minor_version(solidity_version_str);
    let patch_version = get_solidity_patch_version(solidity_version_str);

    assert_eq!(major_version, 0);
    assert_eq!(minor_version, 8);
    assert_eq!(patch_version, 13);
}

#[test]
fn test_get_solidity_minor_version() {
    let solidity_version_str = r#"pragma solidity >=0.8.13;"#;

    let minor_version = get_solidity_minor_version(solidity_version_str);

    assert_eq!(minor_version, 8);
}
