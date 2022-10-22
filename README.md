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




## Gas Optimizations

#### `address_balance`
- Use `selfbalance()` instead of `address(this).balance`.

##### `address_zero`
- Use assembly to check for `address(0)`,

#### `assign_update_array_value`
- When updating a value in an array with arithmetic, using `array[index] += amount` is cheaper than `array[index] = array[index] + amount`. This optimization also catches other arithmetic, bitwise and other operations.

#### `bool_equals_bool`
- Instead of `if (x == bool)`, use `if(x)` or when applicable, use assembly with `iszero(iszero(x))`.

#### `cache_array_length`
- Cache array length during for loops.

#### `constant_variable`
- Mark storage variables as `constant` if they never change and are not marked as constants.


#### `immutable_variable`
- Mark storage variables as `immutable` if variables are assigned during deployment and never change afterwards. 

#### `increment_decrement`
- Use `unchecked{++i}` instead of `i++`, or `++i` (or use assembly when applicable). This also applies to decrementing as well.

#### `memory_to_calldata`
- Use `calldata` for function arguments marked as `memory` that do not get mutated.

#### `multiple_require`
- Use multiple require() statments insted of require(expression && expression && ...)








## Vulnerabilities


## QA