// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

/// @title Liquidity Book Constants Library
/// @author Trader Joe
/// @notice Set of constants for Liquidity Book contracts
library Constants {
    uint256 internal constant SCALE_OFFSET = 128;
    uint256 internal constant SCALE = 1 << SCALE_OFFSET;

    uint256 internal constant PRECISION = 1e18;
    uint256 internal constant BASIS_POINT_MAX = 10_000;
}
