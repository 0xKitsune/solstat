&nbsp;
## 🪲 Identified Vulnerabilities
Below are the currently identified vulnerabilities that solstat identifies. If you would like to check out a list of patterns that are ready to be implemented and you would like to add them to the repo, you can check out the [Contribution.md](Contributing.md#potential-optimizations-vulnerability-and-qa-additions)!

| Vulnerability             | Description                                             |
| ------------------------- | ------------------------------------------------------- |
| unsafe_erc20_operation    | Use `safeTransfer()`, `safeTransferFrom()`, `safeApprove()` instead of ERC20 `transfer()`, `transferFrom()`, `approve()`. |
