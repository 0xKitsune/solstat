mod analyzer;
mod opts;
mod report;

use analyzer::*;
use opts::Opts;
use report::generation::generate_report;

#[macro_use]
extern crate colour;

fn main() {
    let opts = Opts::new();

    let vulnerabilities = vulnerabilities::analyze_dir(&opts.path, opts.vulnerabilities);
    let optimizations = optimizations::analyze_dir(&opts.path, opts.optimizations);
    let qa = qa::analyze_dir(&opts.path, opts.qa);

    generate_report(vulnerabilities, optimizations, qa);
}
