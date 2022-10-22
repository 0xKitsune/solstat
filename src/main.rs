mod analyzer;
mod opts;
mod report;

use std::path;

use clap::Parser;
use opts::Opts;

use crate::opts::Args;
use report::generation::generate_report;

use analyzer::*;

fn main() {
    let opts = Opts::new();

    let vulnerabilities = vulnerabilities::analyze_dir(&opts.path, opts.vulnerabilities);
    let optimizations = optimizations::analyze_dir(&opts.path, opts.optimizations);
    let qa = qa::analyze_dir(&opts.path, opts.qa);

    generate_report(vulnerabilities, optimizations, qa);
}
