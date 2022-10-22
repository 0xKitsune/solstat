# solstat
A Solidity static analyzer to identify contract vulnerabilities and gas efficiencies. 

```js
.------. .------. .------. .------. .------. .------. .------.
|S.--. | |O.--. | |L.--. | |S.--. | |T.--. | |A.--. | |T.--. |
| :/\: | | :/\: | | :/\: | | :/\: | | :/\: | | (\/) | | :/\: |
| :\/: | | :\/: | | (__) | | :\/: | | (__) | | :\/: | | (__) |
| '--'S| | '--'O| | '--'L| | '--'S| | '--'T| | '--'A| | '--'T|
`------' `------' `------' `------' `------' `------' `------'
```

If you would like to quickly jump to any section, you can use the following links.

[Installing Solstat]()

[Running Solstat]()

[Currently Identified Optimizations, Vulnerabilities and QA]()

[Contributing]()


<hr>
<br>

# Installing Solstat
First, make sure that you have [Rust installed](https://www.rust-lang.org/tools/install). Then you can choose either of the installation methods by entering the corresponding command in your terminal below.

### Install from crates.io
```
cargo install solstat
```

### Install from source
```
git clone https://github.com/0xKitsune/solstat &&
cd solstat &&
cargo install --path .
```

<hr>
<br>

# Running Solstat



# Currently Identified Optimizations, Vulnerabilities and QA 
Below are the currently identified optimizations, vulnerabilities and qa patterns that Solstat identifies. If you would like to check out a list of patterns that are ready to be implemented and you would like to add them to the repo, you can check out the [Contribution.md]()!

<br>


## Gas Optimizations

#### address_balance
- Use `selfbalance()` instead of `address(this).balance`.

#### address_zero
- Use assembly to check for `address(0)`.

#### assign_update_array_value
- When updating a value in an array with arithmetic, using `array[index] += amount` is cheaper than `array[index] = array[index] + amount`. This optimization also catches other arithmetic, bitwise and other operations.

#### bool_equals_bool
- Instead of `if (x == bool)`, use `if(x)` or when applicable, use assembly with `iszero(iszero(x))`.

#### cache_array_length
- Cache array length during for loops.

#### constant_variable
- Mark storage variables as `constant` if they never change and are not marked as constants.


#### immutable_variable
- Mark storage variables as `immutable` if variables are assigned during deployment and never change afterwards. 

#### increment_decrement
- Use `unchecked{++i}` instead of `i++`, or `++i` (or use assembly when applicable). This also applies to decrementing as well.

#### memory_to_calldata
- Use `calldata` for function arguments marked as `memory` that do not get mutated.

#### multiple_require
- Use multiple require() statments insted of require(expression && expression && ...).

#### pack_storage_variables
- Tightly pack storage variables for efficient contract storage.

#### pack_struct_variables
- Tightly pack struct variables for efficient contract storage.

#### payable_function
- Mark functions as payable (with discretion).

#### safe_math_post_080
- Identifies when SafeMath is being used if the contract using solidity >= 0.8.0. Using Safemath when using version >= 0.8.0 is redundant and will incur additional gas costs. 

#### safe_math_pre_080
- Identifies when SafeMath is being used if the contract using solidity < 0.8.0. Consider using assembly with overflow/undeflow protection for math (add, sub, mul, div) instead of SafeMath.

#### shift_math
- Right shift or Left shift instead of dividing or multiplying by powers of two.


#### solidity_keccak256
- Use assembly to hash instead of Solidity.

#### solidity_math
- Use assembly for math (add, sub, mul, div).

#### sstore
- Use assembly to write storage values.

#### string_error
- Use custom errors instead of string error messages for contracts using Solidity version >= 0.8.4.


<hr>
<br>

## Vulnerabilities


## QA