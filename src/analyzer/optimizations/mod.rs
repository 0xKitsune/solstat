use std::{collections::HashMap, fs, vec};

use solang_parser::pt::{self, SourceUnit};

use super::utils::LineNumber;

pub mod address_balance;
pub mod address_zero;
pub mod assign_update_array_value;
pub mod cache_array_length;
pub mod constant_variables;
pub mod if_bool_equals_bool;
pub mod immutable_variables;
pub mod increment_decrement;
pub mod memory_to_calldata;
pub mod multiple_require;
pub mod pack_storage_variables;
pub mod pack_struct_variables;
pub mod payable_function;
pub mod safe_math;
pub mod shift_math;
pub mod solidity_keccak256;
pub mod solidity_math;
pub mod sstore;
pub mod string_errors;
mod template;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Optimization {
    AddressBalance,
    AddressZero,
    AssignUpdateArrayValue,
    CacheArrayLength,
    ConstantVariables,
    IfBoolEqualsBool,
    ImmutableVarialbes,
    IncrementDecrement,
    MemoryToCalldata,
    MultipleRequire,
    PackStorageVariables,
    PackStructVariables,
    PayableFunction,
    SafeMathPre080,
    SafeMathPost080,
    ShiftMath,
    SolidityKeccak256,
    SolidityMath,
    Sstore,
    StringErrors,
}

pub fn get_all_optimizations() -> Vec<Optimization> {
    vec![
        Optimization::AddressBalance,
        Optimization::AddressZero,
        Optimization::AssignUpdateArrayValue,
        Optimization::CacheArrayLength,
        Optimization::ConstantVariables,
        Optimization::IfBoolEqualsBool,
        Optimization::ImmutableVarialbes,
        Optimization::IncrementDecrement,
        Optimization::MemoryToCalldata,
        Optimization::MultipleRequire,
        Optimization::PackStorageVariables,
        Optimization::PackStructVariables,
        Optimization::PayableFunction,
        Optimization::SafeMathPre080,
        Optimization::SafeMathPost080,
        Optimization::ShiftMath,
        Optimization::SolidityKeccak256,
        Optimization::SolidityMath,
        Optimization::Sstore,
        Optimization::StringErrors,
    ]
}

pub fn analyze_dir(
    target_dir: &str,
    optimizations: Vec<Optimization>,
) -> HashMap<Optimization, Vec<(String, Vec<i32>)>> {
    //Initialize a new hashmap to keep track of all the optimizations across the target dir
    let mut optimization_locations: HashMap<Optimization, Vec<(String, Vec<i32>)>> = HashMap::new();

    //For each file in the target dir
    for (i, path) in fs::read_dir(target_dir)
        .expect(format!("Could not read contracts from directory: {:?}", target_dir).as_str())
        .into_iter()
        .enumerate()
    {
        //Get the file path, name and contents
        let file_path = path
            .expect(format!("Could not file unwrap path").as_str())
            .path();

        let file_name = file_path
            .file_name()
            .expect("Could not unwrap file name to OsStr")
            .to_str()
            .expect("Could not convert file name from OsStr to &str")
            .to_string();

        let file_contents = fs::read_to_string(&file_path).expect("Unable to read file");

        //For each active optimization
        for optimization in &optimizations {
            let line_numbers = analyze_for_optimization(&file_contents, i, *optimization);

            if line_numbers.len() > 0 {
                let file_optimizations = optimization_locations
                    .entry(optimization.clone())
                    .or_insert(vec![]);

                file_optimizations.push((file_name.clone(), line_numbers));
            }
        }
    }

    optimization_locations
}

pub fn analyze_for_optimization(
    file_contents: &str,
    file_number: usize,
    optimization: Optimization,
) -> Vec<LineNumber> {
    let line_numbers = vec![];

    //Parse the file into a the ast
    let source_unit = solang_parser::parse(file_contents, file_number).unwrap().0;

    line_numbers
}
