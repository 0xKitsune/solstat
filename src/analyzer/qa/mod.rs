pub mod constructor_order;
pub mod template;

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
    path::PathBuf,
    str::FromStr,
};

use self::constructor_order::constructor_order_qa;

use super::utils::{self, LineNumber};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum QualityAssurance {
    ConstructorOrder,
}

pub fn get_all_qa() -> Vec<QualityAssurance> {
    vec![QualityAssurance::ConstructorOrder]
}

pub fn str_to_qa(qa: &str) -> QualityAssurance {
    match qa.to_lowercase().as_str() {
        "constructor_order" => QualityAssurance::ConstructorOrder,
        other => {
            panic!("Unrecgonized qa: {}", other)
        }
    }
}

pub fn analyze_dir(
    target_dir: &str,
    qa: Vec<QualityAssurance>,
) -> HashMap<QualityAssurance, Vec<(String, BTreeSet<LineNumber>)>> {
    //Initialize a new hashmap to keep track of all the optimizations across the target dir
    let mut qa_locations: HashMap<QualityAssurance, Vec<(String, BTreeSet<LineNumber>)>> =
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

        if file_path.is_dir() {
            qa_locations.extend(analyze_dir(
                file_path
                    .as_os_str()
                    .to_str()
                    .expect("Could not get nested dir"),
                qa.clone(),
            ))
        } else {
            let file_name = file_path
                .file_name()
                .expect(format!("Could not unwrap file name to OsStr: {}", i).as_str())
                .to_str()
                .expect("Could not convert file name from OsStr to &str")
                .to_string();

            if file_name.ends_with(".sol") && !file_name.to_lowercase().contains(".t.sol") {
                let file_contents = fs::read_to_string(&file_path).expect("Unable to read file");

                //For each active optimization
                for target in &qa {
                    let line_numbers = analyze_for_qa(&file_contents, i, *target);

                    if line_numbers.len() > 0 {
                        let file_optimizations =
                            qa_locations.entry(target.clone()).or_insert(vec![]);

                        file_optimizations.push((file_name.clone(), line_numbers));
                    }
                }
            }
        }
    }

    qa_locations
}

pub fn analyze_for_qa(
    file_contents: &str,
    file_number: usize,
    qa: QualityAssurance,
) -> BTreeSet<LineNumber> {
    let mut line_numbers: BTreeSet<LineNumber> = BTreeSet::new();

    //Parse the file into a the ast
    let source_unit = solang_parser::parse(&file_contents, file_number).unwrap().0;

    let locations = match qa {
        QualityAssurance::ConstructorOrder => constructor_order_qa(source_unit),
        _ => panic!("Location dont recognized"),
    };

    for loc in locations {
        line_numbers.insert(utils::get_line_number(loc.start(), file_contents));
    }

    line_numbers
}
