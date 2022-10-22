pub mod template;

use std::{collections::HashMap, fs};

use solang_parser::pt::SourceUnit;

use super::utils::LineNumber;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum QualityAssurance {}

pub fn get_all_qa() -> Vec<QualityAssurance> {
    vec![]
}

pub fn str_to_qa(qa: &str) -> QualityAssurance {
    match qa.to_lowercase().as_str() {
        other => {
            panic!("Unrecgonized qa: {}", other)
        }
    }
}

pub fn analyze_dir(
    target_dir: &str,
    qa: Vec<QualityAssurance>,
) -> HashMap<QualityAssurance, Vec<(String, Vec<i32>)>> {
    //Initialize a new hashmap to keep track of all the optimizations across the target dir
    let mut vulnerability_locations: HashMap<QualityAssurance, Vec<(String, Vec<i32>)>> =
        HashMap::new();

    //For each file in the target dir
    for (i, path) in fs::read_dir(target_dir)
        .expect(format!("Could not read contracts from directory: {:?}", target_dir).as_str())
        .into_iter()
        .enumerate()
    {
        //Get the file path, name and contents
        let file_path = path
            .expect(format!("Could not unwrap file path: {}", i).as_str())
            .path();

        let file_name = file_path
            .file_name()
            .expect(format!("Could not unwrap file name to OsStr: {}", i).as_str())
            .to_str()
            .expect("Could not convert file name from OsStr to &str")
            .to_string();

        let file_contents = fs::read_to_string(&file_path).expect("Unable to read file");

        //For each active optimization
        for target in &qa {
            let line_numbers = analyze_for_qa(&file_contents, i, *target);

            if line_numbers.len() > 0 {
                let file_optimizations = vulnerability_locations
                    .entry(target.clone())
                    .or_insert(vec![]);

                file_optimizations.push((file_name.clone(), line_numbers));
            }
        }
    }

    vulnerability_locations
}

pub fn analyze_for_qa(
    file_contents: &str,
    file_number: usize,
    qa: QualityAssurance,
) -> Vec<LineNumber> {
    let line_numbers = vec![];

    //Parse the file into a the ast
    let source_unit = solang_parser::parse(&file_contents, file_number).unwrap().0;

    line_numbers
}
