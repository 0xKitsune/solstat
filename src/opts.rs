use std::{fs, process};

use crate::analyzer::{optimizations, vulnerabilities};
use crate::analyzer::{
    optimizations::{str_to_optimization, Optimization},
    qa::{self, str_to_qa, QualityAssurance},
    vulnerabilities::{str_to_vulnerability, Vulnerability},
};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "Solstat",
    about = "A Solidity static analyzer to identify contract vulnerabilities and gas efficiencies."
)]

pub struct Args {
    #[clap(
        short,
        long,
        help = "Path to the directory containing the files Solstat will analyze. The default directory is `./contracts`"
    )]
    pub path: Option<String>,

    #[clap(
        short,
        long,
        help = "Path to the toml file containing the Solstat configuration when not using the default settings."
    )]
    pub toml: Option<String>,
}

pub struct Opts {
    pub path: String,
    pub optimizations: Vec<Optimization>,
    pub vulnerabilities: Vec<Vulnerability>,
    pub qa: Vec<QualityAssurance>,
}

#[derive(serde::Deserialize, Debug)]
pub struct SolstatToml {
    pub path: String,
    pub optimizations: Vec<String>,
    pub vulnerabilities: Vec<String>,
    pub qa: Vec<String>,
}

impl Opts {
    pub fn new() -> Opts {
        let args = Args::parse();

        let (optimizations, vulnerabilities, qa) = if args.toml.is_some() {
            let toml_path = args.toml.unwrap();

            let toml_str =
                fs::read_to_string(toml_path).expect("Could not read toml file to string");

            let solstat_toml: SolstatToml =
                toml::from_str(&toml_str).expect("Could not convert toml contents to SolstatToml");

            (
                solstat_toml
                    .optimizations
                    .iter()
                    .map(|f| str_to_optimization(f))
                    .collect::<Vec<Optimization>>(),
                solstat_toml
                    .vulnerabilities
                    .iter()
                    .map(|f| str_to_vulnerability(f))
                    .collect::<Vec<Vulnerability>>(),
                solstat_toml
                    .vulnerabilities
                    .iter()
                    .map(|f| str_to_qa(f))
                    .collect::<Vec<QualityAssurance>>(),
            )
        } else {
            (
                optimizations::get_all_optimizations(),
                vulnerabilities::get_all_vulnerabilities(),
                qa::get_all_qa(),
            )
        };

        let path = if args.path.is_some() {
            args.path.unwrap()
        } else {
            match fs::read_dir("./contracts") {
                Ok(_) => {}

                Err(_) => {
                    yellow!(
                        "Error when reading the target contracts directory. 
If the `--path` flag is not passed, Solstat will look for `./contracts` by default.
To fix this, either add a `./contracts` directory or provide `--path <path_to_contracts_dir>\n"
                    );
                    process::exit(1)
                }
            }

            String::from("./contracts")
        };

        Opts {
            path,
            optimizations,
            vulnerabilities,
            qa,
        }
    }
}
