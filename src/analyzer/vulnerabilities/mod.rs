pub mod template;
pub mod unsafe_erc20_operation;

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    env, fs,
    path::PathBuf,
    str::FromStr,
};

use solang_parser::pt::SourceUnit;

use super::utils::{self, LineNumber};

use unsafe_erc20_operation::unsafe_erc20_operation_vulnerability;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]

pub enum Vulnerability {
    UnsafeERC20Operation,
}

pub fn get_all_vulnerabilities() -> Vec<Vulnerability> {
    vec![Vulnerability::UnsafeERC20Operation]
}

pub fn str_to_vulnerability(vuln: &str) -> Vulnerability {
    match vuln.to_lowercase().as_str() {
        "unsafe_erc20_operation" => Vulnerability::UnsafeERC20Operation,

        other => {
            panic!("Unrecgonized vulnerability: {}", other)
        }
    }
}

pub fn analyze_dir(
    target_dir: &str,
    vulnerabilities: Vec<Vulnerability>,
) -> HashMap<Vulnerability, Vec<(String, BTreeSet<LineNumber>)>> {
    //Initialize a new hashmap to keep track of all the optimizations across the target dir
    let mut vulnerability_locations: HashMap<Vulnerability, Vec<(String, BTreeSet<LineNumber>)>> =
        HashMap::new();

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

        if file_path.is_dir() {
            vulnerability_locations.extend(analyze_dir(
                file_path
                    .as_os_str()
                    .to_str()
                    .expect("Could not get nested dir"),
                vulnerabilities.clone(),
            ))
        } else {
            let file_name = file_path
                .file_name()
                .expect("Could not unwrap file name to OsStr")
                .to_str()
                .expect("Could not convert file name from OsStr to &str")
                .to_string();

            if file_name.ends_with(".sol") && !file_name.to_lowercase().contains(".t.sol") {
                let file_contents = fs::read_to_string(&file_path).expect("Unable to read file");

                //For each active optimization
                for vulnerability in &vulnerabilities {
                    let line_numbers = analyze_for_vulnerability(&file_contents, i, *vulnerability);

                    if line_numbers.len() > 0 {
                        let file_optimizations = vulnerability_locations
                            .entry(vulnerability.clone())
                            .or_insert(vec![]);

                        file_optimizations.push((file_name.clone(), line_numbers));
                    }
                }
            }
        }
    }

    vulnerability_locations
}

pub fn analyze_for_vulnerability(
    file_contents: &str,
    file_number: usize,
    vulnerability: Vulnerability,
) -> BTreeSet<LineNumber> {
    let mut line_numbers: BTreeSet<LineNumber> = BTreeSet::new();

    //Parse the file into a the ast
    let source_unit = solang_parser::parse(&file_contents, file_number).unwrap().0;

    let locations = match vulnerability {
        Vulnerability::UnsafeERC20Operation => unsafe_erc20_operation_vulnerability(source_unit),
    };

    for loc in locations {
        line_numbers.insert(utils::get_line_number(loc.start(), file_contents));
    }

    line_numbers
}
