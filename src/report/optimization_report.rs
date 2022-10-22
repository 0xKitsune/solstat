use std::{collections::HashMap, fs};

use crate::analyzer::optimizations::Optimization;

pub fn generate_optimization_report(
    optimizations: HashMap<Optimization, Vec<(String, Vec<i32>)>>,
) -> String {
    let mut optimization_report = String::from("");

    let optimization_report_sections_path: String =
        "./src/report/report_sections/optimizations/".to_owned();

    //Add optimization report overview
    let overview_section =
        fs::read_to_string(optimization_report_sections_path.clone() + "overview.md")
            .expect("Unable to read overview.md");

    optimization_report.push_str((overview_section + "\n").as_str());

    for optimization in optimizations {
        if optimization.1.len() > 0 {
            let optimization_target = optimization.0;

            let report_section = get_optimization_report_section(
                optimization_target,
                optimization_report_sections_path.clone(),
            );

            let mut matches_section = String::from("### Lines\n");

            for (file_name, lines) in optimization.1 {
                for line in lines {
                    //- file_name:line_number\n
                    matches_section
                        .push_str(&(String::from("- ") + &file_name + ":" + &line.to_string()));
                    matches_section.push_str("\n");
                }
            }

            matches_section.push_str("\n\n");

            let completed_report_section = report_section + "\n" + matches_section.as_str();
            optimization_report.push_str(completed_report_section.as_str());
        }
    }

    optimization_report
}

pub fn get_optimization_report_section(
    optimization: Optimization,
    optimization_report_sections_path: String,
) -> String {
    match optimization {
        Optimization::AddressBalance => {
            fs::read_to_string(optimization_report_sections_path + "address_balance.md")
                .expect("Unable to read file")
        }

        Optimization::AddressZero => {
            fs::read_to_string(optimization_report_sections_path + "address_zero.md")
                .expect("Unable to read file")
        }

        Optimization::AssignUpdateArrayValue => {
            fs::read_to_string(optimization_report_sections_path + "assign_update_array_value.md")
                .expect("Unable to read file")
        }

        Optimization::BoolEqualsBool => {
            fs::read_to_string(optimization_report_sections_path + "bool_equals_bool.md")
                .expect("Unable to read file")
        }

        Optimization::CacheArrayLength => {
            fs::read_to_string(optimization_report_sections_path + "cache_array_length.md")
                .expect("Unable to read file")
        }
        Optimization::ConstantVariables => {
            fs::read_to_string(optimization_report_sections_path + "constant_variable.md")
                .expect("Unable to read file")
        }

        Optimization::ImmutableVarialbes => {
            fs::read_to_string(optimization_report_sections_path + "immutable_variable.md")
                .expect("Unable to read file")
        }
        Optimization::IncrementDecrement => {
            fs::read_to_string(optimization_report_sections_path + "increment_decrement.md")
                .expect("Unable to read file")
        }
        Optimization::MemoryToCalldata => {
            fs::read_to_string(optimization_report_sections_path + "memory_to_calldata.md")
                .expect("Unable to read file")
        }
        Optimization::MultipleRequire => {
            fs::read_to_string(optimization_report_sections_path + "multiple_require.md")
                .expect("Unable to read file")
        }
        Optimization::PackStorageVariables => {
            fs::read_to_string(optimization_report_sections_path + "pack_storage_variables.md")
                .expect("Unable to read file")
        }
        Optimization::PackStructVariables => {
            fs::read_to_string(optimization_report_sections_path + "pack_struct_variables.md")
                .expect("Unable to read file")
        }
        Optimization::PayableFunction => {
            fs::read_to_string(optimization_report_sections_path + "payable_function.md")
                .expect("Unable to read file")
        }
        Optimization::SafeMathPre080 => {
            fs::read_to_string(optimization_report_sections_path + "safe_math_pre_080.md")
                .expect("Unable to read file")
        }
        Optimization::SafeMathPost080 => {
            fs::read_to_string(optimization_report_sections_path + "safe_math_post_080.md")
                .expect("Unable to read file")
        }
        Optimization::ShiftMath => {
            fs::read_to_string(optimization_report_sections_path + "shift_math.md")
                .expect("Unable to read file")
        }
        Optimization::SolidityKeccak256 => {
            fs::read_to_string(optimization_report_sections_path + "solidity_keccak256.md")
                .expect("Unable to read file")
        }
        Optimization::SolidityMath => {
            fs::read_to_string(optimization_report_sections_path + "solidity_math.md")
                .expect("Unable to read file")
        }
        Optimization::Sstore => fs::read_to_string(optimization_report_sections_path + "sstore.md")
            .expect("Unable to read file"),

        Optimization::StringErrors => {
            fs::read_to_string(optimization_report_sections_path + "string_errors.md")
                .expect("Unable to read file")
        }
    }
}
