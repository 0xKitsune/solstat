pub fn report_section_content() -> String {
    String::from(
        r##" 
## No use of underscore for internal and private variable names | Don't use the underscore prefix for public variable names

Prefix `internal` and `private` variable names with an underscore in order to follow `Style Guides Rules` (ref: https://github.com/protofire/solhint/blob/master/docs/rules/naming/private-vars-leading-underscore.md).
In the other hand, public variable names must not have an underscore prefix.

```js
// Bad
contract Contract0 {
    address public _owner;
    uint256 public _num1;
}

// Good
contract Contract1 {
    address public owner;
    uint256 public num1;
}
```
    "##,
    )
}
