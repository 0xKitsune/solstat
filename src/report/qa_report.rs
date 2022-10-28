use std::{collections::HashMap, fs};

use crate::analyzer::qa::QualityAssurance;
use crate::report::report_sections::qa::overview;

pub fn generate_qa_report(qa_items: HashMap<QualityAssurance, Vec<(String, Vec<i32>)>>) -> String {
    let mut qa_report = String::from("");

    //Add optimization report overview
    let overview_section = overview::report_section_content();

    qa_report.push_str((overview_section + "\n").as_str());

    for item in qa_items {
        if item.1.len() > 0 {
            let qa_target = item.0;
            let matches = item.1;

            let report_section = get_qa_report_section(qa_target);

            let mut matches_section = String::from("### Lines\n");

            for (file_name, mut lines) in matches {
                lines.dedup();
                lines.sort();

                for line in lines {
                    //- file_name:line_number\n
                    matches_section
                        .push_str(&(String::from("- ") + &file_name + ":" + &line.to_string()));
                    matches_section.push_str("\n");
                }
            }

            matches_section.push_str("\n\n");

            let completed_report_section = report_section + "\n" + matches_section.as_str();
            qa_report.push_str(completed_report_section.as_str());
        }
    }

    qa_report
}

pub fn get_qa_report_section(qa: QualityAssurance) -> String {
    match qa {}
}
