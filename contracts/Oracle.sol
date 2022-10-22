// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "../LBErrors.sol";
import "./Buffer.sol";
import "./Samples.sol";

/// @title Liquidity Book Oracle Library
/// @author Trader Joe
/// @notice Helper contract for oracle
library Oracle {
    using Samples for bytes32;
    using Buffer for uint256;

    struct Sample {
        uint256 timestamp;
        uint256 cumulativeId;
        uint256 cumulativeVolatilityAccumulated;
        uint256 cumulativeBinCrossed;
    }

    /// @notice View function to get the oracle's sample at `_ago` seconds
    /// @dev Return a linearized sample, the weighted average of 2 neighboring samples
    /// @param _oracle The oracle storage pointer
    /// @param _activeSize The size of the oracle (without empty data)
    /// @param _activeId The active index of the oracle
    /// @param _lookUpTimestamp The looked up date
    /// @return timestamp The timestamp of the sample
    /// @return cumulativeId The weighted average cumulative id
    /// @return cumulativeVolatilityAccumulated The weighted average cumulative volatility accumulated
    /// @return cumulativeBinCrossed The weighted average cumulative bin crossed
    function getSampleAt(
        bytes32[65_535] storage _oracle,
        uint256 _activeSize,
        uint256 _activeId,
        uint256 _lookUpTimestamp
    )
        internal
        view
        returns (
            uint256 timestamp,
            uint256 cumulativeId,
            uint256 cumulativeVolatilityAccumulated,
            uint256 cumulativeBinCrossed
        )
    {
        if (_activeSize == 0) revert Oracle__NotInitialized();

        // Oldest sample
        uint256 _nextId;
        assembly {
            _nextId := addmod(_activeId, 1, _activeSize)
        }
        bytes32 _sample = _oracle[_nextId];
        timestamp = _sample.timestamp();
        if (timestamp > _lookUpTimestamp) revert Oracle__LookUpTimestampTooOld(timestamp, _lookUpTimestamp);

        // Most recent sample
        if (_activeSize != 1) {
            _sample = _oracle[_activeId];
            timestamp = _sample.timestamp();

            if (timestamp > _lookUpTimestamp) {
                bytes32 _next;
                (_sample, _next) = binarySearch(_oracle, _activeId, _lookUpTimestamp, _activeSize);

                if (_sample != _next) {
                    uint256 _weightPrev = _next.timestamp() - _lookUpTimestamp; // _next.timestamp() - _sample.timestamp() - (_lookUpTimestamp - _sample.timestamp())
                    uint256 _weightNext = _lookUpTimestamp - _sample.timestamp(); // _next.timestamp() - _sample.timestamp() - (_next.timestamp() - _lookUpTimestamp)
                    uint256 _totalWeight = _weightPrev + _weightNext; // _next.timestamp() - _sample.timestamp()

                    cumulativeId =
                        (_sample.cumulativeId() * _weightPrev + _next.cumulativeId() * _weightNext) /
                        _totalWeight;
                    cumulativeVolatilityAccumulated =
                        (_sample.cumulativeVolatilityAccumulated() *
                            _weightPrev +
                            _next.cumulativeVolatilityAccumulated() *
                            _weightNext) /
                        _totalWeight;
                    cumulativeBinCrossed =
                        (_sample.cumulativeBinCrossed() * _weightPrev + _next.cumulativeBinCrossed() * _weightNext) /
                        _totalWeight;
                    return (_lookUpTimestamp, cumulativeId, cumulativeVolatilityAccumulated, cumulativeBinCrossed);
                }
            }
        }

        timestamp = _sample.timestamp();
        cumulativeId = _sample.cumulativeId();
        cumulativeVolatilityAccumulated = _sample.cumulativeVolatilityAccumulated();
        cumulativeBinCrossed = _sample.cumulativeBinCrossed();
    }

    /// @notice Function to update a sample
    /// @param _oracle The oracle storage pointer
    /// @param _size The size of the oracle (last ids can be empty)
    /// @param _sampleLifetime The lifetime of a sample, it accumulates information for up to this timestamp
    /// @param _lastTimestamp The timestamp of the creation of the oracle's latest sample
    /// @param _lastIndex The index of the oracle's latest sample
    /// @param _activeId The active index of the pair during the latest swap
    /// @param _volatilityAccumulated The volatility accumulated of the pair during the latest swap
    /// @param _binCrossed The bin crossed during the latest swap
    /// @return updatedIndex The oracle updated index, it is either the same as before, or the next one
    function update(
        bytes32[65_535] storage _oracle,
        uint256 _size,
        uint256 _sampleLifetime,
        uint256 _lastTimestamp,
        uint256 _lastIndex,
        uint256 _activeId,
        uint256 _volatilityAccumulated,
        uint256 _binCrossed
    ) internal returns (uint256 updatedIndex) {
        bytes32 _updatedPackedSample = _oracle[_lastIndex].update(_activeId, _volatilityAccumulated, _binCrossed);

        if (block.timestamp - _lastTimestamp >= _sampleLifetime && _lastTimestamp != 0) {
            assembly {
                updatedIndex := addmod(_lastIndex, 1, _size)
            }
        } else updatedIndex = _lastIndex;

        _oracle[updatedIndex] = _updatedPackedSample;
    }

    /// @notice Initialize the sample
    /// @param _oracle The oracle storage pointer
    /// @param _index The index to initialize
    function initialize(bytes32[65_535] storage _oracle, uint256 _index) internal {
        _oracle[_index] |= bytes32(uint256(1));
    }

    /// @notice Binary search on oracle samples and return the 2 samples (as bytes32) that surrounds the `lookUpTimestamp`
    /// @dev The oracle needs to be in increasing order `{_index + 1, _index + 2 ..., _index + _activeSize} % _activeSize`.
    /// The sample that aren't initialized yet will be skipped as _activeSize only contains the samples that are initialized.
    /// This function works only if `timestamp(_oracle[_index + 1 % _activeSize] <= _lookUpTimestamp <= timestamp(_oracle[_index]`.
    /// The edge cases needs to be handled before
    /// @param _oracle The oracle storage pointer
    /// @param _index The current index of the oracle
    /// @param _lookUpTimestamp The looked up timestamp
    /// @param _activeSize The size of the oracle (without empty data)
    /// @return prev The last sample with a timestamp lower than the lookUpTimestamp
    /// @return next The first sample with a timestamp greater than the lookUpTimestamp
    function binarySearch(
        bytes32[65_535] storage _oracle,
        uint256 _index,
        uint256 _lookUpTimestamp,
        uint256 _activeSize
    ) private view returns (bytes32 prev, bytes32 next) {
        // The sample with the lowest timestamp is the one right after _index
        uint256 _low = 1;
        uint256 _high = _activeSize;

        uint256 _middle;
        uint256 _id;

        bytes32 _sample;
        uint256 _sampleTimestamp;
        while (_high >= _low) {
            unchecked {
                _middle = (_low + _high) / 2;
                assembly {
                    _id := addmod(_middle, _index, _activeSize)
                }
                _sample = _oracle[_id];
                _sampleTimestamp = _sample.timestamp();
                if (_sampleTimestamp < _lookUpTimestamp) {
                    _low = _middle + 1;
                } else if (_sampleTimestamp > _lookUpTimestamp) {
                    _high = _middle - 1;
                } else {
                    return (_sample, _sample);
                }
            }
        }
        if (_sampleTimestamp < _lookUpTimestamp) {
            assembly {
                _id := addmod(_id, 1, _activeSize)
            }
            (prev, next) = (_sample, _oracle[_id]);
        } else (prev, next) = (_oracle[_id.before(_activeSize)], _sample);
    }
}
