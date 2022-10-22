// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

/// @title Liquidity Book Buffer Library
/// @author Trader Joe
/// @notice Helper contract used for modulo calculation
library Buffer {
    /// @notice Internal function to do positive (x - 1) % n
    /// @param x The value
    /// @param n The modulo value
    /// @return result The result
    function before(uint256 x, uint256 n) internal pure returns (uint256 result) {
        assembly {
            if gt(n, 0) {
                switch x
                case 0 {
                    result := sub(n, 1)
                }
                default {
                    result := mod(sub(x, 1), n)
                }
            }
        }
    }
}
