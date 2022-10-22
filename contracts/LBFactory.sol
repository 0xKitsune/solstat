// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "openzeppelin/proxy/Clones.sol";
import "openzeppelin/utils/structs/EnumerableSet.sol";

import "./LBErrors.sol";
import "./libraries/Constants.sol";
import "./libraries/Decoder.sol";
import "./libraries/PendingOwnable.sol";
import "./libraries/SafeCast.sol";
import "./interfaces/ILBFactory.sol";

/// @title Liquidity Book Factory
/// @author Trader Joe
/// @notice Contract used to deploy and register new LBPairs.
/// Enables setting fee parameters, flashloan fees and LBPair implementation.
/// Unless the `creationUnlocked` is `true`, only the owner of the factory can create pairs.
contract LBFactory is PendingOwnable, ILBFactory {
    using SafeCast for uint256;
    using Decoder for bytes32;
    using EnumerableSet for EnumerableSet.AddressSet;

    uint256 public constant override MAX_FEE = 0.1e18; // 10%

    uint256 public constant override MIN_BIN_STEP = 1; // 0.01%
    uint256 public constant override MAX_BIN_STEP = 100; // 1%, can't be greater than 247 for indexing reasons

    uint256 public constant override MAX_PROTOCOL_SHARE = 2_500; // 25%

    address public override LBPairImplementation;

    address public override feeRecipient;

    uint256 public override flashLoanFee;

    /// @notice Whether the createLBPair function is unlocked and can be called by anyone (true) or only by owner (false)
    bool public override creationUnlocked;

    ILBPair[] public override allLBPairs;

    /// @dev Mapping from a (tokenA, tokenB, binStep) to a LBPair. The tokens are ordered to save gas, but they can be
    /// in the reverse order in the actual pair. Always query one of the 2 tokens of the pair to assert the order of the 2 tokens
    mapping(IERC20 => mapping(IERC20 => mapping(uint256 => LBPairInformation))) private _LBPairsInfo;

    // Whether a preset was set or not, if the bit at `index` is 1, it means that the binStep `index` was set
    // The max binStep set is 247. We use this method instead of an array to keep it ordered and to reduce gas
    bytes32 private _availablePresets;

    // The parameters presets
    mapping(uint256 => bytes32) private _presets;

    EnumerableSet.AddressSet private _quoteAssetWhitelist;

    // Whether a LBPair was created with a bin step, if the bit at `index` is 1, it means that the LBPair with binStep `index` exists
    // The max binStep set is 247. We use this method instead of an array to keep it ordered and to reduce gas
    mapping(IERC20 => mapping(IERC20 => bytes32)) private _availableLBPairBinSteps;

    /// @notice Constructor
    /// @param _feeRecipient The address of the fee recipient
    /// @param _flashLoanFee The value of the fee for flash loan
    constructor(address _feeRecipient, uint256 _flashLoanFee) {
        _setFeeRecipient(_feeRecipient);

        flashLoanFee = _flashLoanFee;
        emit FlashLoanFeeSet(0, _flashLoanFee);
    }

    /// @notice View function to return the number of LBPairs created
    /// @return The number of LBPair
    function getNumberOfLBPairs() external view override returns (uint256) {
        return allLBPairs.length;
    }

    /// @notice View function to return the number of quote assets whitelisted
    /// @return The number of quote assets
    function getNumberOfQuoteAssets() external view override returns (uint256) {
        return _quoteAssetWhitelist.length();
    }

    /// @notice View function to return the quote asset whitelisted at index `index`
    /// @param _index The index
    /// @return The address of the _quoteAsset at index `index`
    function getQuoteAsset(uint256 _index) external view override returns (IERC20) {
        return IERC20(_quoteAssetWhitelist.at(_index));
    }

    /// @notice View function to return whether a token is a quotedAsset (true) or not (false)
    /// @param _token The address of the asset
    /// @return Whether the token is a quote asset or not
    function isQuoteAsset(IERC20 _token) external view override returns (bool) {
        return _quoteAssetWhitelist.contains(address(_token));
    }

    /// @notice Returns the LBPairInformation if it exists,
    /// if not, then the address 0 is returned. The order doesn't matter
    /// @param _tokenA The address of the first token of the pair
    /// @param _tokenB The address of the second token of the pair
    /// @param _binStep The bin step of the LBPair
    /// @return The LBPairInformation
    function getLBPairInformation(
        IERC20 _tokenA,
        IERC20 _tokenB,
        uint256 _binStep
    ) external view override returns (LBPairInformation memory) {
        return _getLBPairInformation(_tokenA, _tokenB, _binStep);
    }

    /// @notice View function to return the different parameters of the preset
    /// @param _binStep The bin step of the preset
    /// @return baseFactor The base factor
    /// @return filterPeriod The filter period of the preset
    /// @return decayPeriod The decay period of the preset
    /// @return reductionFactor The reduction factor of the preset
    /// @return variableFeeControl The variable fee control of the preset
    /// @return protocolShare The protocol share of the preset
    /// @return maxVolatilityAccumulated The max volatility accumulated of the preset
    /// @return sampleLifetime The sample lifetime of the preset
    function getPreset(uint16 _binStep)
        external
        view
        override
        returns (
            uint256 baseFactor,
            uint256 filterPeriod,
            uint256 decayPeriod,
            uint256 reductionFactor,
            uint256 variableFeeControl,
            uint256 protocolShare,
            uint256 maxVolatilityAccumulated,
            uint256 sampleLifetime
        )
    {
        bytes32 _preset = _presets[_binStep];
        if (_preset == bytes32(0)) revert LBFactory__BinStepHasNoPreset(_binStep);

        uint256 _shift;

        // Safety check
        assert(_binStep == _preset.decode(type(uint16).max, _shift));

        baseFactor = _preset.decode(type(uint16).max, _shift += 16);
        filterPeriod = _preset.decode(type(uint16).max, _shift += 16);
        decayPeriod = _preset.decode(type(uint16).max, _shift += 16);
        reductionFactor = _preset.decode(type(uint16).max, _shift += 16);
        variableFeeControl = _preset.decode(type(uint24).max, _shift += 16);
        protocolShare = _preset.decode(type(uint16).max, _shift += 24);
        maxVolatilityAccumulated = _preset.decode(type(uint24).max, _shift += 16);

        sampleLifetime = _preset.decode(type(uint16).max, 240);
    }

    /// @notice View function to return the list of available binStep with a preset
    /// @return presetsBinStep The list of binStep
    function getAllBinSteps() external view override returns (uint256[] memory presetsBinStep) {
        unchecked {
            bytes32 _avPresets = _availablePresets;
            uint256 _nbPresets = _avPresets.decode(type(uint8).max, 248);

            if (_nbPresets > 0) {
                presetsBinStep = new uint256[](_nbPresets);

                uint256 _index;
                for (uint256 i = MIN_BIN_STEP; i <= MAX_BIN_STEP; ++i) {
                    if (_avPresets.decode(1, i) == 1) {
                        presetsBinStep[_index] = i;
                        if (++_index == _nbPresets) break;
                    }
                }
            }
        }
    }

    /// @notice View function to return all the LBPair of a pair of tokens
    /// @param _tokenX The first token of the pair
    /// @param _tokenY The second token of the pair
    /// @return LBPairsAvailable The list of available LBPairs
    function getAllLBPairs(IERC20 _tokenX, IERC20 _tokenY)
        external
        view
        override
        returns (LBPairInformation[] memory LBPairsAvailable)
    {
        unchecked {
            (IERC20 _tokenA, IERC20 _tokenB) = _sortTokens(_tokenX, _tokenY);

            bytes32 _avLBPairBinSteps = _availableLBPairBinSteps[_tokenA][_tokenB];
            uint256 _nbAvailable = _avLBPairBinSteps.decode(type(uint8).max, 248);

            if (_nbAvailable > 0) {
                LBPairsAvailable = new LBPairInformation[](_nbAvailable);

                uint256 _index;
                for (uint256 i = MIN_BIN_STEP; i <= MAX_BIN_STEP; ++i) {
                    if (_avLBPairBinSteps.decode(1, i) == 1) {
                        LBPairInformation memory _LBPairInformation = _LBPairsInfo[_tokenA][_tokenB][i];

                        LBPairsAvailable[_index] = LBPairInformation({
                            binStep: i.safe24(),
                            LBPair: _LBPairInformation.LBPair,
                            createdByOwner: _LBPairInformation.createdByOwner,
                            ignoredForRouting: _LBPairInformation.ignoredForRouting
                        });
                        if (++_index == _nbAvailable) break;
                    }
                }
            }
        }
    }

    /// @notice Set the LBPair implementation address
    /// @dev Needs to be called by the owner
    /// @param _LBPairImplementation The address of the implementation
    function setLBPairImplementation(address _LBPairImplementation) external override onlyOwner {
        if (ILBPair(_LBPairImplementation).factory() != this)
            revert LBFactory__LBPairSafetyCheckFailed(_LBPairImplementation);

        address _oldLBPairImplementation = LBPairImplementation;
        if (_oldLBPairImplementation == _LBPairImplementation)
            revert LBFactory__SameImplementation(_LBPairImplementation);

        LBPairImplementation = _LBPairImplementation;

        emit LBPairImplementationSet(_oldLBPairImplementation, _LBPairImplementation);
    }

    /// @notice Create a liquidity bin LBPair for _tokenX and _tokenY
    /// @param _tokenX The address of the first token
    /// @param _tokenY The address of the second token
    /// @param _activeId The active id of the pair
    /// @param _binStep The bin step in basis point, used to calculate log(1 + binStep)
    /// @return _LBPair The address of the newly created LBPair
    function createLBPair(
        IERC20 _tokenX,
        IERC20 _tokenY,
        uint24 _activeId,
        uint16 _binStep
    ) external override returns (ILBPair _LBPair) {
        address _owner = owner();
        if (!creationUnlocked && msg.sender != _owner) revert LBFactory__FunctionIsLockedForUsers(msg.sender);

        address _LBPairImplementation = LBPairImplementation;

        if (_LBPairImplementation == address(0)) revert LBFactory__ImplementationNotSet();

        if (!_quoteAssetWhitelist.contains(address(_tokenY))) revert LBFactory__QuoteAssetNotWhitelisted(_tokenY);

        if (_tokenX == _tokenY) revert LBFactory__IdenticalAddresses(_tokenX);

        // We sort token for storage efficiency, only one input needs to be stored
        (IERC20 _tokenA, IERC20 _tokenB) = _sortTokens(_tokenX, _tokenY);
        // single check is sufficient
        if (address(_tokenA) == address(0)) revert LBFactory__AddressZero();
        if (address(_LBPairsInfo[_tokenA][_tokenB][_binStep].LBPair) != address(0))
            revert LBFactory__LBPairAlreadyExists(_tokenX, _tokenY, _binStep);

        bytes32 _preset = _presets[_binStep];
        if (_preset == bytes32(0)) revert LBFactory__BinStepHasNoPreset(_binStep);

        uint256 _sampleLifetime = _preset.decode(type(uint16).max, 240);
        // We remove the bits that are not part of the feeParameters
        _preset &= bytes32(uint256(type(uint144).max));

        bytes32 _salt = keccak256(abi.encode(_tokenA, _tokenB, _binStep));
        _LBPair = ILBPair(Clones.cloneDeterministic(_LBPairImplementation, _salt));

        _LBPair.initialize(_tokenX, _tokenY, _activeId, uint16(_sampleLifetime), _preset);

        _LBPairsInfo[_tokenA][_tokenB][_binStep] = LBPairInformation({
            binStep: _binStep,
            LBPair: _LBPair,
            createdByOwner: msg.sender == _owner,
            ignoredForRouting: false
        });

        allLBPairs.push(_LBPair);

        {
            bytes32 _avLBPairBinSteps = _availableLBPairBinSteps[_tokenA][_tokenB];
            // We add a 1 at bit `_binStep` as this binStep is now set
            _avLBPairBinSteps = bytes32(uint256(_avLBPairBinSteps) | (1 << _binStep));

            // Increase the number of lb pairs by 1
            _avLBPairBinSteps = bytes32(uint256(_avLBPairBinSteps) + (1 << 248));

            // Save the changes
            _availableLBPairBinSteps[_tokenA][_tokenB] = _avLBPairBinSteps;
        }

        emit LBPairCreated(_tokenX, _tokenY, _binStep, _LBPair, allLBPairs.length - 1);

        emit FeeParametersSet(
            msg.sender,
            _LBPair,
            _binStep,
            _preset.decode(type(uint16).max, 16),
            _preset.decode(type(uint16).max, 32),
            _preset.decode(type(uint16).max, 48),
            _preset.decode(type(uint16).max, 64),
            _preset.decode(type(uint24).max, 80),
            _preset.decode(type(uint16).max, 104),
            _preset.decode(type(uint24).max, 120)
        );
    }

    /// @notice Function to set whether the pair is ignored or not for routing, it will make the pair unusable by the router
    /// @param _tokenX The address of the first token of the pair
    /// @param _tokenY The address of the second token of the pair
    /// @param _binStep The bin step in basis point of the pair
    /// @param _ignored Whether to ignore (true) or not (false) the pair for routing
    function setLBPairIgnored(
        IERC20 _tokenX,
        IERC20 _tokenY,
        uint256 _binStep,
        bool _ignored
    ) external override onlyOwner {
        (IERC20 _tokenA, IERC20 _tokenB) = _sortTokens(_tokenX, _tokenY);

        LBPairInformation memory _LBPairInformation = _LBPairsInfo[_tokenA][_tokenB][_binStep];
        if (address(_LBPairInformation.LBPair) == address(0)) revert LBFactory__AddressZero();

        if (_LBPairInformation.ignoredForRouting == _ignored) revert LBFactory__LBPairIgnoredIsAlreadyInTheSameState();

        _LBPairsInfo[_tokenA][_tokenB][_binStep].ignoredForRouting = _ignored;

        emit LBPairIgnoredStateChanged(_LBPairInformation.LBPair, _ignored);
    }

    /// @notice Sets the preset parameters of a bin step
    /// @param _binStep The bin step in basis point, used to calculate log(1 + binStep)
    /// @param _baseFactor The base factor, used to calculate the base fee, baseFee = baseFactor * binStep
    /// @param _filterPeriod The period where the accumulator value is untouched, prevent spam
    /// @param _decayPeriod The period where the accumulator value is halved
    /// @param _reductionFactor The reduction factor, used to calculate the reduction of the accumulator
    /// @param _variableFeeControl The variable fee control, used to control the variable fee, can be 0 to disable them
    /// @param _protocolShare The share of the fees received by the protocol
    /// @param _maxVolatilityAccumulated The max value of the volatility accumulated
    /// @param _sampleLifetime The lifetime of an oracle's sample
    function setPreset(
        uint16 _binStep,
        uint16 _baseFactor,
        uint16 _filterPeriod,
        uint16 _decayPeriod,
        uint16 _reductionFactor,
        uint24 _variableFeeControl,
        uint16 _protocolShare,
        uint24 _maxVolatilityAccumulated,
        uint16 _sampleLifetime
    ) external override onlyOwner {
        bytes32 _packedFeeParameters = _getPackedFeeParameters(
            _binStep,
            _baseFactor,
            _filterPeriod,
            _decayPeriod,
            _reductionFactor,
            _variableFeeControl,
            _protocolShare,
            _maxVolatilityAccumulated
        );

        // The last 16 bits are reserved for sampleLifetime
        bytes32 _preset = bytes32(
            (uint256(_packedFeeParameters) & type(uint144).max) | (uint256(_sampleLifetime) << 240)
        );

        _presets[_binStep] = _preset;

        bytes32 _avPresets = _availablePresets;
        if (_avPresets.decode(1, _binStep) == 0) {
            // We add a 1 at bit `_binStep` as this binStep is now set
            _avPresets = bytes32(uint256(_avPresets) | (1 << _binStep));

            // Increase the number of preset by 1
            _avPresets = bytes32(uint256(_avPresets) + (1 << 248));

            // Save the changes
            _availablePresets = _avPresets;
        }

        emit PresetSet(
            _binStep,
            _baseFactor,
            _filterPeriod,
            _decayPeriod,
            _reductionFactor,
            _variableFeeControl,
            _protocolShare,
            _maxVolatilityAccumulated,
            _sampleLifetime
        );
    }

    /// @notice Remove the preset linked to a binStep
    /// @param _binStep The bin step to remove
    function removePreset(uint16 _binStep) external override onlyOwner {
        if (_presets[_binStep] == bytes32(0)) revert LBFactory__BinStepHasNoPreset(_binStep);

        // Set the bit `_binStep` to 0
        bytes32 _avPresets = _availablePresets;

        _avPresets &= bytes32(type(uint256).max - (1 << _binStep));
        _avPresets = bytes32(uint256(_avPresets) - (1 << 248));

        // Save the changes
        _availablePresets = _avPresets;
        delete _presets[_binStep];

        emit PresetRemoved(_binStep);
    }

    /// @notice Function to set the fee parameter of a LBPair
    /// @param _tokenX The address of the first token
    /// @param _tokenY The address of the second token
    /// @param _binStep The bin step in basis point, used to calculate log(1 + binStep)
    /// @param _baseFactor The base factor, used to calculate the base fee, baseFee = baseFactor * binStep
    /// @param _filterPeriod The period where the accumulator value is untouched, prevent spam
    /// @param _decayPeriod The period where the accumulator value is halved
    /// @param _reductionFactor The reduction factor, used to calculate the reduction of the accumulator
    /// @param _variableFeeControl The variable fee control, used to control the variable fee, can be 0 to disable them
    /// @param _protocolShare The share of the fees received by the protocol
    /// @param _maxVolatilityAccumulated The max value of volatility accumulated
    function setFeesParametersOnPair(
        IERC20 _tokenX,
        IERC20 _tokenY,
        uint16 _binStep,
        uint16 _baseFactor,
        uint16 _filterPeriod,
        uint16 _decayPeriod,
        uint16 _reductionFactor,
        uint24 _variableFeeControl,
        uint16 _protocolShare,
        uint24 _maxVolatilityAccumulated
    ) external override onlyOwner {
        ILBPair _LBPair = _getLBPairInformation(_tokenX, _tokenY, _binStep).LBPair;

        if (address(_LBPair) == address(0)) revert LBFactory__LBPairNotCreated(_tokenX, _tokenY, _binStep);

        bytes32 _packedFeeParameters = _getPackedFeeParameters(
            _binStep,
            _baseFactor,
            _filterPeriod,
            _decayPeriod,
            _reductionFactor,
            _variableFeeControl,
            _protocolShare,
            _maxVolatilityAccumulated
        );

        _LBPair.setFeesParameters(_packedFeeParameters);

        emit FeeParametersSet(
            msg.sender,
            _LBPair,
            _binStep,
            _baseFactor,
            _filterPeriod,
            _decayPeriod,
            _reductionFactor,
            _variableFeeControl,
            _protocolShare,
            _maxVolatilityAccumulated
        );
    }

    /// @notice Function to set the recipient of the fees. This address needs to be able to receive ERC20s
    /// @param _feeRecipient The address of the recipient
    function setFeeRecipient(address _feeRecipient) external override onlyOwner {
        _setFeeRecipient(_feeRecipient);
    }

    /// @notice Function to set the flash loan fee
    /// @param _flashLoanFee The value of the fee for flash loan
    function setFlashLoanFee(uint256 _flashLoanFee) external override onlyOwner {
        uint256 _oldFlashLoanFee = flashLoanFee;

        if (_oldFlashLoanFee == _flashLoanFee) revert LBFactory__SameFlashLoanFee(_flashLoanFee);

        flashLoanFee = _flashLoanFee;
        emit FlashLoanFeeSet(_oldFlashLoanFee, _flashLoanFee);
    }

    /// @notice Function to set the creation restriction of the Factory
    /// @param _locked If the creation is restricted (true) or not (false)
    function setFactoryLockedState(bool _locked) external override onlyOwner {
        if (creationUnlocked != _locked) revert LBFactory__FactoryLockIsAlreadyInTheSameState();
        creationUnlocked = !_locked;
        emit FactoryLockedStatusUpdated(_locked);
    }

    /// @notice Function to add an asset to the whitelist of quote assets
    /// @param _quoteAsset The quote asset (e.g: AVAX, USDC...)
    function addQuoteAsset(IERC20 _quoteAsset) external override onlyOwner {
        if (!_quoteAssetWhitelist.add(address(_quoteAsset)))
            revert LBFactory__QuoteAssetAlreadyWhitelisted(_quoteAsset);

        emit QuoteAssetAdded(_quoteAsset);
    }

    /// @notice Function to remove an asset to the whitelist of quote assets
    /// @param _quoteAsset The quote asset (e.g: AVAX, USDC...)
    function removeQuoteAsset(IERC20 _quoteAsset) external override onlyOwner {
        if (!_quoteAssetWhitelist.remove(address(_quoteAsset))) revert LBFactory__QuoteAssetNotWhitelisted(_quoteAsset);

        emit QuoteAssetRemoved(_quoteAsset);
    }

    /// @notice Internal function to set the recipient of the fee
    /// @param _feeRecipient The address of the recipient
    function _setFeeRecipient(address _feeRecipient) internal {
        if (_feeRecipient == address(0)) revert LBFactory__AddressZero();

        address _oldFeeRecipient = feeRecipient;
        if (_oldFeeRecipient == _feeRecipient) revert LBFactory__SameFeeRecipient(_feeRecipient);

        feeRecipient = _feeRecipient;
        emit FeeRecipientSet(_oldFeeRecipient, _feeRecipient);
    }

    function forceDecay(ILBPair _LBPair) external override onlyOwner {
        _LBPair.forceDecay();
    }

    /// @notice Internal function to set the fee parameter of a LBPair
    /// @param _binStep The bin step in basis point, used to calculate log(1 + binStep)
    /// @param _baseFactor The base factor, used to calculate the base fee, baseFee = baseFactor * binStep
    /// @param _filterPeriod The period where the accumulator value is untouched, prevent spam
    /// @param _decayPeriod The period where the accumulator value is halved
    /// @param _reductionFactor The reduction factor, used to calculate the reduction of the accumulator
    /// @param _variableFeeControl The variable fee control, used to control the variable fee, can be 0 to disable them
    /// @param _protocolShare The share of the fees received by the protocol
    /// @param _maxVolatilityAccumulated The max value of volatility accumulated
    function _getPackedFeeParameters(
        uint16 _binStep,
        uint16 _baseFactor,
        uint16 _filterPeriod,
        uint16 _decayPeriod,
        uint16 _reductionFactor,
        uint24 _variableFeeControl,
        uint16 _protocolShare,
        uint24 _maxVolatilityAccumulated
    ) private pure returns (bytes32) {
        if (_binStep < MIN_BIN_STEP || _binStep > MAX_BIN_STEP)
            revert LBFactory__BinStepRequirementsBreached(MIN_BIN_STEP, _binStep, MAX_BIN_STEP);

        if (_baseFactor > Constants.BASIS_POINT_MAX)
            revert LBFactory__BaseFactorOverflows(_baseFactor, Constants.BASIS_POINT_MAX);

        if (_filterPeriod >= _decayPeriod) revert LBFactory__DecreasingPeriods(_filterPeriod, _decayPeriod);

        if (_reductionFactor > Constants.BASIS_POINT_MAX)
            revert LBFactory__ReductionFactorOverflows(_reductionFactor, Constants.BASIS_POINT_MAX);

        if (_protocolShare > MAX_PROTOCOL_SHARE)
            revert LBFactory__ProtocolShareOverflows(_protocolShare, MAX_PROTOCOL_SHARE);

        {
            uint256 _baseFee = (uint256(_baseFactor) * _binStep) * 1e10;

            // Can't overflow as the max value is `max(uint24) * (max(uint24) * max(uint16)) ** 2 < max(uint104)`
            // It returns 18 decimals as:
            // decimals(variableFeeControl * (volatilityAccumulated * binStep)**2 / 100) = 4 + (4 + 4) * 2 - 2 = 18
            uint256 _prod = uint256(_maxVolatilityAccumulated) * _binStep;
            uint256 _maxVariableFee = (_prod * _prod * _variableFeeControl) / 100;

            if (_baseFee + _maxVariableFee > MAX_FEE)
                revert LBFactory__FeesAboveMax(_baseFee + _maxVariableFee, MAX_FEE);
        }

        /// @dev It's very important that the sum of the sizes of those values is exactly 256 bits
        /// here, (112 + 24) + 16 + 24 + 16 + 16 + 16 + 16 + 16 = 256
        return
            bytes32(
                abi.encodePacked(
                    uint136(_maxVolatilityAccumulated), // The first 112 bits are reserved for the dynamic parameters
                    _protocolShare,
                    _variableFeeControl,
                    _reductionFactor,
                    _decayPeriod,
                    _filterPeriod,
                    _baseFactor,
                    _binStep
                )
            );
    }

    /// @notice Returns the LBPairInformation if it exists,
    /// if not, then the address 0 is returned. The order doesn't matter
    /// @param _tokenA The address of the first token of the pair
    /// @param _tokenB The address of the second token of the pair
    /// @param _binStep The bin step of the LBPair
    /// @return The LBPairInformation
    function _getLBPairInformation(
        IERC20 _tokenA,
        IERC20 _tokenB,
        uint256 _binStep
    ) private view returns (LBPairInformation memory) {
        (_tokenA, _tokenB) = _sortTokens(_tokenA, _tokenB);
        return _LBPairsInfo[_tokenA][_tokenB][_binStep];
    }

    /// @notice Private view function to sort 2 tokens in ascending order
    /// @param _tokenA The first token
    /// @param _tokenB The second token
    /// @return The sorted first token
    /// @return The sorted second token
    function _sortTokens(IERC20 _tokenA, IERC20 _tokenB) private pure returns (IERC20, IERC20) {
        if (_tokenA > _tokenB) (_tokenA, _tokenB) = (_tokenB, _tokenA);
        return (_tokenA, _tokenB);
    }
}
