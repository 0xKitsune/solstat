use crate::analyzer::{optimizations, vulnerabilities};
use crate::analyzer::{
    optimizations::Optimization,
    qa::{self, QualityAssurance},
    vulnerabilities::Vulnerability,
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

    #[clap(
        short,
        long,
        help = "Path of the directory where Solstat reports will be written to."
    )]
    pub out: Option<String>,
}

pub struct Opts {
    pub path: String,
    pub out: String,
    pub optimizations: Vec<Optimization>,
    pub vulnerabilities: Vec<Vulnerability>,
    pub qa: Vec<QualityAssurance>,
}

impl Opts {
    pub fn new() -> Opts {
        let args = Args::parse();

        let (optimizations, vulnerabilities, qa) = if args.toml.is_some() {
            (vec![], vec![], vec![])
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
            String::from("./contracts")
        };

        let out = if args.out.is_some() {
            args.out.unwrap()
        } else {
            String::from("solstat_reports")
        };

        Opts {
            path,
            out,
            optimizations,
            vulnerabilities,
            qa,
        }
    }
}
