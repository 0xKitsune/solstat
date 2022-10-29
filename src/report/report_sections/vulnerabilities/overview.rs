pub fn report_section_content(total_vulnerabilities: usize) -> String {
    String::from(
        format!("# Gas Optimizations - (Total Vulnerabilities {})\n
The following sections detail the high, medium and low severity vulnerabilities found throughout the codebase.\n\n<br>\n
", total_vulnerabilities)
    )
}
