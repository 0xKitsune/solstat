&nbsp;
## 🪲 Identified Vulnerabilities
Below are the currently identified vulnerabilities that solstat identifies. If you would like to check out a list of patterns that are ready to be implemented and you would like to add them to the repo, you can check out the [Contribution.md](https://github.com/0xKitsune/solstat/blob/main/docs/Contributing.md#potential-optimizations-vulnerability-and-qa-additions)!

| Vulnerability             | Description                                             |
| ------------------------- | ------------------------------------------------------- |
| divide_before_multiply    | Use multiplication symbol before division symbol |
| floating_pragma           | Use locked pragma rather than floating pragma |
| unprotected_selfdestruct  | Add sufficient access control to methods that call `selfdestruct` |
| unsafe_erc20_operation    | Use `safeTransfer()`, `safeTransferFrom()`, `safeApprove()` instead of ERC20 `transfer()`, `transferFrom()`, `approve()`. |
