use regex::Regex;
use solang_parser::pt;

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
