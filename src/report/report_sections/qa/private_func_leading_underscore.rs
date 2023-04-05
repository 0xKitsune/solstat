pub fn report_section_content() -> String {
    String::from(
        r##"
## No use of underscore for internal and private function names | Don't use the underscore prefix for public and external function names

Prefix `internal` and `private` function names with an underscore in order to follow `Style Guides Rules` (ref: https://github.com/protofire/solhint/blob/master/docs/rules/naming/private-vars-leading-underscore.md).
In the other hand, public and external function names must not have an underscore prefix.

```js
// Bad
contract Contract0 {
   
    function msgSender() private view returns (address) {
        return msg.sender;
    }

    function msgData() internal view returns (bytes calldata) {
        return msg.data;
    }

    function _currentTimestamp() public view returns(uint256) {
        return block.timestamp;
    }

    function _currentBlockhash() external view returns(uint256) {
        return block.blockhash;
    }
}

// Good
contract Contract1 {
    
    function _msgSender() private view returns (address) {
        return msg.sender;
    }

    function _msgData() internal view returns (bytes calldata) {
        return msg.data;
    }

    function currentTimestamp() public view returns(uint256) {
        return block.timestamp;
    }

    function currentBlockhash() external view returns(uint256) {
        return block.blockhash;
    }
}
```
    "##,
    )
}
