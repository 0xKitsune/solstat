pub fn report_section_content() -> String {
    String::from(
        r##"
Consider ordering multiplication before division to avoid loss of precision because integer division might truncate. Loss of precision in Solidity can lead to vulnerabilities because it can result in unexpected behavior in smart contracts. This can be particularly problematic in financial applications, where even small errors in calculations can have significant consequences. For example, if a contract uses integer division to calculate a result and the division operation truncates the fractional part of the result, it could lead to incorrect pricing or loss of funds due to miscalculated balances.

#### Unsafe Division
```js
n = 5 / 2 * 4; // n = 8 because 5 / 2 == 2 since division truncates.
```
#### Safe Division
```js
n = 5 * 4 / 2; // n = 10
```

"##,
    )
}
