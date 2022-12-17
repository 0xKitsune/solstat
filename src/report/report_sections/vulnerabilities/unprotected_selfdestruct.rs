pub fn report_section_content() -> String {
    String::from(
        r##"
Unprotected call to a function executing `selfdestruct` or `suicide`.

#### Exploit scenario
```js
contract Suicidal {
  function kill() public {
      selfdestruct(msg.sender);
  }
}
```
Anyone can call kill() and destroy the contract.

#### Recommendations
Protect access to all affected functions. Consider one of the following solutions:
1. Restrict the visibility of the function to `internal` or `private`. 
2. If the function must be public, either:
  2.1. Add a modifier to allow only shortlisted EOAs to call this function (such as `onlyOwner`).
  2.2. Add a check on the `msg.sender` directly inside the affected function.

```js
  // restrict visibility to internal or private
  function kill() internal {
    selfdestruct(msg.sender);
  }

  // add a modifier to allow only shortlisted EOAs to call this function
  function kill() public onlyOwner {
    selfdestruct(msg.sender);
  }

  // add a check on the msg.sender directly inside the affected function
  function kill() public {
    require(msg.sender == owner);
    selfdestruct(msg.sender);
  }
```
"##,
    )
}
