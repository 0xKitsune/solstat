pub fn report_section_content() -> String {
    String::from(
        r##"
Floating pragma is a vulnerability in smart contract code that can cause unexpected behavior by allowing the compiler to use a specified range of versions. This can lead to issues such as using an older compiler version with known vulnerabilities, using a newer compiler version with undiscovered vulnerabilities, inconsistency across files using different versions, or unpredictable behavior because the compiler can use any version within the specified range. It is recommended to use a locked pragma version in order to avoid these potential vulnerabilities.

In some cases it may be acceptable to use a floating pragma, such as when a contract is intended for consumption by other developers and needs to be compatible with a range of compiler versions.

#### Bad
```js
pragma solidity ^0.8.0;
```
#### Good
```js
pragma solidity 0.8.15;
```

"##,
    )
}
