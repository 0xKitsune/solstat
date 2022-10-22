# Contributing To Solstat
Thanks for checking out the `Contribution.md`! Contributions are welcomed and encouraged. Below are the guidelines for contributions.

1.) Before starting to work on a PR, check the github issues as well as the PRs to make sure that someone has not already PRed the addition you are thinking of contributing. If someone has already started work on a specific issue, feel free to send a message in the issue thread to see what the status of the PR is. 

2.) Open up a github issue for the contribution. Feel free to ask any questions about the implementation or different parts of the codebase. This is a great place to refine ideas before implementing the changes and submitting a PR.

3.) PR to the `development` branch and link the github issue. From there, the PR will be reviewed and any edits that are necessary will be suggested. Once all edits are complete and the ci pipline clears, your contribution will be merged! Shortly after merging to `development`, the development branch will then be merged to `main`.


The repository can seem a little dense in some parts but adding a new optimization, vulnerability or qa pattern is incredibly simple! Below is a quick walkthrough on how to add each.

<br>

## Optimizations

### Adding the Optimization
All optimizations are located in `src/analyzer/optimizations`. Here you will see a new file for each of the optimizations that Solstat looks for. To add a new optimization, start by adding a new file in this directory  (ex. `pack_struct_variables.rs` would be the file name for the optimization that analyzes for struct packing).

Now that you have a new file for your optimization, copy and paste the code from [`src/analyzer/optimizations/template.rs`]() into your file. 

Lets take a look at a barebones version of the template without any comments.

```rust
pub fn _template_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    let optimization_locations: HashSet<Loc> = HashSet::new();
    let target_nodes = ast::extract_target_from_node(Target::None, source_unit.into());
    for _node in target_nodes {

    }
    optimization_locations
}

#[test]
fn test_template_optimization() {
    let file_contents = r#"
    contract Contract0 {}
    "#;
    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;
    let optimization_locations = _template_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 0)
}

```
Each optimization must take one argument called `source_unit` which is of type `SourceUnit`. Solstat uses the `solang-parser` crate to parse Solidity contracts. The `SourceUnit` type is the resulting type from `solang_parser::parse()` which you will see later in the test case. This function must also return a `Hashset<Loc>`, with the `Loc` type also being from the `solang-parser` crate. The `Loc` type represents a location in the file being analyzed.

Solstat works under the hood by analyzing an abstract syntax tree representing a Solidity contract for specific patterns that you want to find. For example, if you wanted to find all expressions that use addition in a contract, you could look for the `Target::Add` within the AST.

`SourceUnit` is the root node in an Abstract Syntax Tree created from parsing the contract with `solang_parser::parse()`. Helper functions like `ast::extract_target_from_node` and `ast::extract_targets_from_node` are located in the `Solstat::analyzer::ast` module to extract specfic nodes from the AST. The return value of these functions is `Vec<Node>`, with a `Node` representing a node in the AST.

This might sound a little complicated but its way easier than it sounds. Once we look at a full example this will make much more sense. 

Once all of the target nodes are extracted, you can traverse the node for specfic patterns that indicate a match in the pattern you are looking for.

For some easy to read examples, checkout:
- [`src/analyzer/optimizations/address_zero.rs`]()
- [`src/analyzer/optimizations/multiple_require.rs`]()
- [`src/analyzer/optimizations/solidity_keccak256.rs`]()

### Writing a test
Now that you have the optimization logic, make sure to write a test suite at the bottom of the file. The template has all the necessary building blocks you need so that you only need to supply the Solidity code, and how many findings the optimization should identify.


### Updating the codebase
Now that the tests are passing, you are in the home stretch! The last thing you need to do is update the codebase to include your optimization. Here are the steps to do so.


Head to `src/analyzer/optimizations/mod.rs` as all these changes will be in this file.


First add your new mod along side the other `pub mod <mod_name>`.

```rust
pub mod address_balance;
pub mod address_zero;
pub mod assign_update_array_value;
pub mod bool_equals_bool;
pub mod cache_array_length;
pub mod constant_variables;
//--snip--
pub mod <your_mod_here>;
```

Next add your optimization function alongside the other imported optimization functions.

```rust
use self::{
    address_balance::address_balance_optimization,
    address_zero::address_zero_optimization,
    assign_update_array_value::assign_update_array_optimization,
    bool_equals_bool::bool_equals_bool_optimization,
    //--snip--
    your_mod_name::your_function_name;
```


Then add your optimization to the `Optimization` enum.

```rust
pub enum Optimization {
    AddressBalance,
    AddressZero,
    AssignUpdateArrayValue,
    CacheArrayLength,
    ConstantVariables,
    //--snip--
    YourOptimizationHere
```

Add your optimization variant to the return vec in `get_all_optimizations`.

```rust

pub fn get_all_optimizations() -> Vec<Optimization> {
    vec![
        Optimization::AddressBalance,
        Optimization::AddressZero,
        Optimization::AssignUpdateArrayValue,
        Optimization::CacheArrayLength,
        //--snip--
        Optimization::YourOptimizationHere
```

Almost there, two more things! Add your optimization to `str_to_optimization()`
```rust
pub fn str_to_optimization(opt: &str) -> Optimization {
    match opt.to_lowercase().as_str() {
        "address_balance" => Optimization::AddressBalance,
        "address_zero" => Optimization::AddressZero,
        "assign_update_array_value" => Optimization::AssignUpdateArrayValue,
        "cache_array_length" => Optimization::CacheArrayLength,
        "constant_variables" => Optimization::ConstantVariables,
        //--snip--
        "your_optimization_here" => Optimization::YourOptimizationHere,

```

And finally, add pattern matching for your optimization and function to `analyze_for_optimization()`!

```rust

pub fn analyze_for_optimization(
    file_contents: &str,
    file_number: usize,
    optimization: Optimization,
) -> Vec<LineNumber> {
    let mut line_numbers = vec![];

    //Parse the file into a the ast
    let source_unit = solang_parser::parse(file_contents, file_number).unwrap().0;

    let locations = match optimization {
        Optimization::AddressBalance => address_balance_optimization(source_unit),
        Optimization::AddressZero => address_zero_optimization(source_unit),
        Optimization::AssignUpdateArrayValue => assign_update_array_optimization(source_unit),
        Optimization::CacheArrayLength => cache_array_length_optimization(source_unit),
        //--snip--
         Optimization::YourOptimizationHere => your_function_name(source_unit),
```


Congrats, you have updated the codebase to implement your optimization!


### Report Section
The final step in the contribution process is to write a report section that describes your optimization. All reports for optimizations are added to `src/report/report_sections/optimizations`. For a template report you can check out [`src/report/report_sections/optimizations/template.md`](). 

Once you have written your report section, the final step before PRing the contribution is to link your report to your optimization by adding pattern matching for your optimization to `get_optimization_report_section()`

```rust

pub fn get_optimization_report_section(
    optimization: Optimization,
    optimization_report_sections_path: String,
) -> String {
    match optimization {
        Optimization::AddressBalance => {
            fs::read_to_string(optimization_report_sections_path + "address_balance.md")
                .expect("Unable to read file")
        }

        Optimization::AddressZero => {
            fs::read_to_string(optimization_report_sections_path + "address_zero.md")
                .expect("Unable to read file")
        }

        //--snip--
        Optimization::YourOptimizationHere => {
            fs::read_to_string(optimization_report_sections_path + "your_report_name.md")
                .expect("Unable to read file")
        }

```


And that wraps up everything. You can now PR to `developement` and wait for the merge!

<br>

## Vulnerabilities


<br>

## QA
