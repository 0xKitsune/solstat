// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "../LBErrors.sol";
import "./BitMath.sol";

/// @title Liquidity Book Tree Math Library
/// @author Trader Joe
/// @notice Helper contract used for finding closest bin with liquidity
library TreeMath {
    using BitMath for uint256;

    /// @notice Returns the first id that is non zero, corresponding to a bin with
    /// liquidity in it
    /// @param _tree The storage slot of the tree
    /// @param _binId the binId to start searching
    /// @param _rightSide Whether we're searching in the right side of the tree (true) or the left side (false)
    /// for the closest non zero bit on the right or the left
    /// @return The closest non zero bit on the right (or left) side of the tree
    function findFirstBin(
        mapping(uint256 => uint256)[3] storage _tree,
        uint24 _binId,
        bool _rightSide
    ) internal view returns (uint24) {
        unchecked {
            uint256 current;
            uint256 bit;

            (_binId, bit) = _getIdsFromAbove(_binId);

            // Search in depth 2
            if ((_rightSide && bit != 0) || (!_rightSide && bit != 255)) {
                current = _tree[2][_binId];
                bit = current.closestBit(uint8(bit), _rightSide);

                if (bit != type(uint256).max) {
                    return _getBottomId(_binId, uint24(bit));
                }
            }

            (_binId, bit) = _getIdsFromAbove(_binId);

            // Search in depth 1
            if ((_rightSide && bit != 0) || (!_rightSide && bit != 255)) {
                current = _tree[1][_binId];
                bit = current.closestBit(uint8(bit), _rightSide);

                if (bit != type(uint256).max) {
                    _binId = _getBottomId(_binId, uint24(bit));
                    current = _tree[2][_binId];

                    return _getBottomId(_binId, current.significantBit(_rightSide));
                }
            }

            // Search in depth 0
            current = _tree[0][0];
            bit = current.closestBit(uint8(_binId), _rightSide);
            if (bit == type(uint256).max) revert TreeMath__ErrorDepthSearch();

            current = _tree[1][bit];
            _binId = _getBottomId(uint24(bit), current.significantBit(_rightSide));

            current = _tree[2][_binId];
            return _getBottomId(_binId, current.significantBit(_rightSide));
        }
    }

    function addToTree(mapping(uint256 => uint256)[3] storage _tree, uint256 _id) internal {
        // add 1 at the right indices
        uint256 _idDepth2 = _id >> 8;
        uint256 _idDepth1 = _id >> 16;

        _tree[2][_idDepth2] |= 1 << (_id & 255);
        _tree[1][_idDepth1] |= 1 << (_idDepth2 & 255);
        _tree[0][0] |= 1 << _idDepth1;
    }

    function removeFromTree(mapping(uint256 => uint256)[3] storage _tree, uint256 _id) internal {
        unchecked {
            // removes 1 at the right indices
            uint256 _idDepth2 = _id >> 8;
            // Optimization of `_tree[2][_idDepth2] & (type(uint256).max - (1 << (_id & 255)))`
            uint256 _newLeafValue = _tree[2][_idDepth2] & (type(uint256).max ^ (1 << (_id & 255)));
            _tree[2][_idDepth2] = _newLeafValue;
            if (_newLeafValue == 0) {
                uint256 _idDepth1 = _id >> 16;
                // Optimization of `_tree[1][_idDepth1] & (type(uint256).max - (1 << (_idDepth2 & 255)))`
                _newLeafValue = _tree[1][_idDepth1] & (type(uint256).max ^ (1 << (_idDepth2 & 255)));
                _tree[1][_idDepth1] = _newLeafValue;
                if (_newLeafValue == 0) {
                    // Optimization of `type(uint256).max - (1 << _idDepth1)`
                    _tree[0][0] &= type(uint256).max ^ (1 << _idDepth1);
                }
            }
        }
    }

    /// @notice Private pure function to return the ids from above
    /// @param _id The current id
    /// @return The branch id from above
    /// @return The leaf id from above
    function _getIdsFromAbove(uint24 _id) private pure returns (uint24, uint24) {
        // Optimization of `(_id / 256, _id % 256)`
        return (_id >> 8, _id & 255);
    }

    /// @notice Private pure function to return the bottom id
    /// @param _branchId The branch id
    /// @param _leafId The leaf id
    /// @return The bottom branchId
    function _getBottomId(uint24 _branchId, uint24 _leafId) private pure returns (uint24) {
        // Optimization of `_branchId * 256 + _leafId`
        // Can't overflow as _leafId would fit in uint8, but kept as uint24 to optimize castings
        unchecked {
            return (_branchId << 8) + _leafId;
        }
    }
}
