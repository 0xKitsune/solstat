use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

use crate::analyzer::{
    optimizations::Optimization, qa::QualityAssurance, utils::LineNumber,
    vulnerabilities::Vulnerability,
};

use super::{
    optimization_report::generate_optimization_report, qa_report::generate_qa_report,
    vulnerability_report::generate_vulnerability_report,
};

pub fn generate_report(
    vulnerabilities: HashMap<Vulnerability, Vec<(String, BTreeSet<LineNumber>)>>,
    optimizations: HashMap<Optimization, Vec<(String, BTreeSet<LineNumber>)>>,
    qa: HashMap<QualityAssurance, Vec<(String, BTreeSet<LineNumber>)>>,
) {
    let mut solstat_report = String::from("");

    if vulnerabilities.len() > 0 {
        solstat_report.push_str(&generate_vulnerability_report(vulnerabilities));
        solstat_report.push_str("\n\n");
    }

    if optimizations.len() > 0 {
        solstat_report.push_str(&generate_optimization_report(optimizations));
        solstat_report.push_str("\n\n");
    }

    if qa.len() > 0 {
        solstat_report.push_str(&generate_qa_report(qa));
        solstat_report.push_str("\n\n");
    }

    fs::write("solstat_report.md", solstat_report).expect("Unable to solstat_report to file");
}
