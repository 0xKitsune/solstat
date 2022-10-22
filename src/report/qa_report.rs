use std::{collections::HashMap, fs};

use crate::analyzer::qa::QualityAssurance;

pub fn generate_qa_report(qa_items: HashMap<QualityAssurance, Vec<(String, Vec<i32>)>>) -> String {
    let mut qa_report = String::from("");

    let qa_report_sections_path: String = "./src/report/report_sections/optimizations/".to_owned();

    //Add optimization report overview
    let overview_section = fs::read_to_string(qa_report_sections_path.clone() + "overview.md")
        .expect("Unable to read overview.md");

    qa_report.push_str((overview_section + "\n").as_str());

    for item in qa_items {
        if item.1.len() > 0 {
            let qa_target = item.0;

            let report_section = get_qa_report_section(qa_target, qa_report_sections_path.clone());

            let mut matches_section = String::from("### Lines\n");

            for (file_name, lines) in item.1 {
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

pub fn get_qa_report_section(qa: QualityAssurance, qa_report_sections_path: String) -> String {
    match qa {}
}
