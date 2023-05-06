pub fn report_section_content() -> String {
    String::from(
        r##" 
## Arbitrary `from` in transferFrom/safeTransferFrom.

Use `msg.sender` as `from` in `transferFrom/safeTransferFrom`.

### Exploit Scenario:

```js
/** 
 * Alice approves this contract to spend her ERC20
 * tokens. Bob can call a and specify Alice's address as
 * the from parameter in transferFrom, allowing him to
 * transfer Alice's tokens to himself.
 * */

function withdraw(address from, address to, uint256 amount) public {
    erc20.transferFrom(from, to, am);
}

// or

function emergencyWithdraw(address from, address to, uint256 amount) public {
    erc20.safeTransferFrom(from, to, am);
}

```
    "##,
    )
}
