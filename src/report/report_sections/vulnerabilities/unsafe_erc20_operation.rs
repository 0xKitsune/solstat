pub fn report_section_content() -> String {
    String::from(
        r##"
ERC20 operations can be unsafe due to different implementations and vulnerabilities in the standard. To account for this, either use OpenZeppelin's SafeERC20 library or wrap each operation in a require statement.
Additionally, ERC20's approve functions have a known race-condition vulnerability. To account for this, use OpenZeppelin's SafeERC20 library's `safeIncrease` or `safeDecrease` Allowance functions.

#### Unsafe Transfer
```js
IERC20(token).transfer(msg.sender, amount);
```
#### OpenZeppelin SafeTransfer
```js
import {SafeERC20} from "openzeppelin/token/utils/SafeERC20.sol";
//--snip--

IERC20(token).safeTransfer(msg.sender, address(this), amount);
```

#### Safe Transfer with require statement.
```js
bool success = IERC20(token).transfer(msg.sender, amount);
require(success, "ERC20 transfer failed");
```

#### Unsafe TransferFrom
```js
IERC20(token).transferFrom(msg.sender, address(this), amount);
```
#### OpenZeppelin SafeTransferFrom
```js
import {SafeERC20} from "openzeppelin/token/utils/SafeERC20.sol";
//--snip--

IERC20(token).safeTransferFrom(msg.sender, address(this), amount);
```

#### Safe TransferFrom with require statement.
```js
bool success = IERC20(token).transferFrom(msg.sender, address(this), amount);
require(success, "ERC20 transfer failed");
```

"##,
    )
}
