use std::{collections::HashMap, fs};

use crate::analyzer::optimizations::Optimization;

use crate::report::report_sections::optimizations::{
    address_balance, address_zero, assign_update_array_value, bool_equals_bool, cache_array_length,
    constant_variable, immutable_variable, increment_decrement, memory_to_calldata,
    multiple_require, overview, pack_storage_variables, pack_struct_variables, payable_function, private_constant,
    safe_math_post_080, safe_math_pre_080, shift_math, solidity_keccak256, solidity_math, sstore,
    string_errors,
};


pub fn generate_optimization_report(
    optimizations: HashMap<Optimization, Vec<(String, Vec<i32>)>>,
) -> String {
    let mut optimization_report = String::from("");

    let mut total_optimizations_found = 0;

    for optimization in optimizations {
        if optimization.1.len() > 0 {
            let optimization_target = optimization.0;

            let report_section = get_optimization_report_section(optimization_target);

            let mut matches_section = String::from("### Lines\n");

            for (file_name, lines) in optimization.1 {
                for line in lines {
                    //- file_name:line_number\n
                    matches_section
                        .push_str(&(String::from("- ") + &file_name + ":" + &line.to_string()));
                    matches_section.push_str("\n");

                    total_optimizations_found += 1;
                }
            }

            matches_section.push_str("\n\n");

            let completed_report_section = report_section + "\n" + matches_section.as_str();
            optimization_report.push_str(completed_report_section.as_str());
        }
    }

    //Add overview to optimization report
    let mut completed_optimization_report =
        overview::report_section_content(total_optimizations_found);

    completed_optimization_report.push_str(optimization_report.as_str());

    completed_optimization_report
}

pub fn get_optimization_report_section(optimization: Optimization) -> String {
    match optimization {
        Optimization::AddressBalance => address_balance::report_section_content(),
        Optimization::AddressZero => address_zero::report_section_content(),
        Optimization::AssignUpdateArrayValue => assign_update_array_value::report_section_content(),
        Optimization::BoolEqualsBool => bool_equals_bool::report_section_content(),
        Optimization::CacheArrayLength => cache_array_length::report_section_content(),
        Optimization::ConstantVariables => constant_variable::report_section_content(),
        Optimization::ImmutableVarialbes => immutable_variable::report_section_content(),
        Optimization::IncrementDecrement => increment_decrement::report_section_content(),
        Optimization::MemoryToCalldata => memory_to_calldata::report_section_content(),
        Optimization::MultipleRequire => multiple_require::report_section_content(),
        Optimization::PackStorageVariables => pack_storage_variables::report_section_content(),
        Optimization::PackStructVariables => pack_struct_variables::report_section_content(),
        Optimization::PayableFunction => payable_function::report_section_content(),
        Optimization::PrivateConstant => private_constant::report_section_content(),
        Optimization::SafeMathPre080 => safe_math_pre_080::report_section_content(),
        Optimization::SafeMathPost080 => safe_math_post_080::report_section_content(),
        Optimization::ShiftMath => shift_math::report_section_content(),
        Optimization::SolidityKeccak256 => solidity_keccak256::report_section_content(),
        Optimization::SolidityMath => solidity_math::report_section_content(),
        Optimization::Sstore => sstore::report_section_content(),
        Optimization::StringErrors => string_errors::report_section_content(),
    }
}
