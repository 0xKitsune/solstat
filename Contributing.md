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

Now that you have a new file for your optimization, copy and paste the code from [`src/analyzer/optimizations/template.rs`](https://github.com/0xKitsune/solstat/blob/main/src/analyzer/optimizations/template.rs) into your file. 

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
- [`src/analyzer/optimizations/address_balance.rs`](https://github.com/0xKitsune/solstat/blob/main/src/analyzer/optimizations/address_balance.rs)
- [`src/analyzer/optimizations/multiple_require.rs`](https://github.com/0xKitsune/solstat/blob/main/src/analyzer/optimizations/multiple_require.rs)
- [`src/analyzer/optimizations/solidity_keccak256.rs`](https://github.com/0xKitsune/solstat/blob/main/src/analyzer/optimizations/solidity_keccak256.rs)

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
The final step in the contribution process is to write a report section that describes your optimization. All reports for optimizations are added to `src/report/report_sections/optimizations`. For a template report you can check out [`src/report/report_sections/optimizations/template.md`](https://github.com/0xKitsune/solstat/blob/main/src/report/report_sections/optimizations/template.md). To generate a quick gas report, feel free to use [0xKitsune/gas-lab](https://github.com/0xKitsune/gas-lab) or set up a environment within Foundry to test your gas comparison. 

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

## QA
Contributing to QA is exactly the same as Optimizations, with the only difference being that any directory path containing `optimizations`, should now contain `qa` instead (ex. `src/analyzer/optimizations/mod.rs` => `src/analyzer/qa/mod.rs`). Everything else is exactly the same as adding an optimization.

<br>

## Vulnerabilities

Contributing to Vulnerabilities is exactly the same as Optimizations, with the two minor differences. The first being any directory path containing `optimizations`, should now contain `vulnerabilities` instead (ex. `src/analyzer/optimizations/mod.rs` => `src/analyzer/vulnerabilities/mod.rs`). 

The second difference is that within `get_vulnerability_report_section()` instead of just returning the file name of the report section, it should also return a `VulnerabilitySeverity`.

```rust
pub enum VulnerabilitySeverity {
    High,
    Medium,
    Low,
}

pub fn get_vulnerability_report_section(
    vulnerability: Vulnerability,
    vulnerability_report_sections_path: String,
) -> (String, VulnerabilitySeverity) {
   
}
```

<br>


# Potential Optimizations, Vulnerability and QA Additions
Below is a non-exhaustive list of potential features to contribute. If you have an optimization, vulnerability or qa pattern you would like to contribute, please open up an issue on the Github repo!

## Low level / QA

[N-02] PUBLIC FUNCTIONS NOT CALLED BY THE CONTRACT SHOULD BE DECLARED EXTERNAL INSTEAD

[N-04] REDUNDANT CAST

[N-07] USE A MORE RECENT VERSION OF SOLIDITY
Use a solidity version of at least 0.8.4 to get bytes.concat() instead of abi.encodePacked(<bytes>,<bytes>)
Use a solidity version of at least 0.8.12 to get string.concat() instead of abi.encodePacked(<str>,<str>)

[N-08] VARIABLE NAMES THAT CONSIST OF ALL CAPITAL LETTERS SHOULD BE RESERVED FOR CONST/IMMUTABLE VARIABLES

[N-09] NATSPEC IS INCOMPLETE

[N-07] FILE IS MISSING NATSPEC

[N-10] EVENT IS MISSING INDEXED FIELDS

[N-12] REMOVE COMMENTED OUT CODE

[N-10] TYPOS IN COMMENTS

[L-09] OPEN TODOS
Code architecture, incentives, and error handling/reporting questions/issues should be resolved before deployment

[L-08] MISSING Zero CHECKS FOR ADDRESS(0X0) WHEN ASSIGNING VALUES TO ADDRESS STATE VARIABLES

[L-08] MISSING Zero CHECKS FOR Uint256 WHEN ASSIGNING VALUES TO uint256 STATE VARIABLES
  


[L-07] SAFEAPPROVE() IS DEPRECATED
Deprecated in favor of safeIncreaseAllowance() and safeDecreaseAllowance(). If only setting the initial allowance to the value that means infinite, safeIncreaseAllowance() can be used instead

[L-03] MISSING CHECKS FOR APPROVE()’S RETURN STATUS
Some tokens, such as Tether (USDT) return false rather than reverting if the approval fails. Use OpenZeppelin’s safeApprove(), which reverts if there’s a failure, instead

[L-07] NO event is raised when Major state variable is changed

[N-04] UNUSED FILE
  
[N-06] VARIABLE NAMES THAT CONSIST OF ALL CAPITAL LETTERS SHOULD BE RESERVED FOR CONST/IMMUTABLE VARIABLES
  
N-07 Large multiples of ten should use scientific notation (e.g. 1e6) rather than decimal literals (e.g. 1000000), for readability
 
[L-07] ABI.ENCODEPACKED() SHOULD NOT BE USED WITH DYNAMIC TYPES WHEN PASSING THE RESULT TO A HASH FUNCTION SUCH AS KECCAK256()
Use abi.encode() instead which will pad items to 32 bytes, which will prevent hash collisions (e.g. abi.encodePacked(0x123,0x456) => 0x123456 => abi.encodePacked(0x1,0x23456), but abi.encode(0x123,0x456) => 0x0...1230...456). “Unless there is a compelling reason, abi.encode should be preferred”. If there is only one argument to abi.encodePacked() it can often be cast to bytes() or bytes32() instead.
  
[L-03] UNUSED RECEIVE() FUNCTION WILL LOCK ETHER IN CONTRACT ?
If the intention is for the Ether to be used, the function should call another function, otherwise it should revert

[N-04] ADDING A RETURN STATEMENT WHEN THE FUNCTION DEFINES A NAMED RETURN VARIABLE, IS REDUNDANT

[L-01] REQUIRE() SHOULD BE USED INSTEAD OF ASSERT()

[L-03] UNBOUNDED LOOPS WITH EXTERNAL CALLS
The interface and the function should require a start index and a lenght, so that the index composition can be fetched in batches without running out of gas. If there are thousands of index components (e.g. like the Wilshire 5000 index), the function may revert

[N-10] SOLIDITY VERSIONS GREATER THAN THE CURRENT VERSION SHOULD NOT BE INCLUDED IN THE PRAGMA RANGE

[N-19] FILE DOES NOT CONTAIN AN SPDX IDENTIFIER

[N-25] NOW IS DEPRECATED

[N-02] REQUIRE()/REVERT() STATEMENTS SHOULD HAVE AN ERROR STRING 
require statments like `require(condition)` should have an error string like `require(condition, "some message")` 

[N-09] THE NONREENTRANT MODIFIER SHOULD OCCUR BEFORE ALL OTHER MODIFIERS
This is a best-practice to protect against reentrancy in other modifiers

[L-03] UNSAFE CALLS TO OPTIONAL ERC20 FUNCTIONS
decimals(), name() and symbol() are optional parts of the ERC20 specification, so there are tokens that do not implement them. It’s not safe to cast arbitrary token addresses in order to call these functions. If IERC20Metadata is to be relied on, that should be the variable type of the token variable, rather than it being address, so the compiler can verify that types correctly match, rather than this being a runtime failure. See this prior instance of this issue which was marked as Low risk. Do this to resolve the issue.

[L-01] CROSS-CHAIN REPLAY ATTACKS
Storing the block.chainid is not safe. See this issue from a prior contest for details.
https://github.com/code-423n4/2021-04-maple-findings/issues/2

[N-04] USE A MORE RECENT VERSION OF SOLIDITY
Use a solidity version of at least 0.8.12 to get string.concat() to be used instead of abi.encodePacked(<str>,<str>)

[L-05] MISSING CONTRACT-EXISTENCE CHECKS BEFORE LOW-LEVEL CALLS
Low-level calls return success if there is no code present at the specified address. In addition to the zero-address checks, add a check to verify that <address>.code.length > 0


  
## Medium

[M-06] ORACLE DATA FEED IS INSUFFICIENTLY VALIDATED
  
[M-01] CALL() SHOULD BE USED INSTEAD OF TRANSFER() ON AN ADDRESS PAYABLE
The use of the deprecated transfer() function for an address will inevitably make the transaction fail when:
- The claimer smart contract does not implement a payable function.
- The claimer smart contract does implement a payable fallback which uses more than 2300 gas unit.
- The claimer smart contract implements a payable fallback function that needs less than 2300 gas units but is called through proxy, raising the call’s gas usage above 2300.
- Additionally, using higher than 2300 gas might be mandatory for some multisig wallets.

