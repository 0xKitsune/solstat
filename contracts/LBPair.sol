// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

/** Imports **/

import "./LBErrors.sol";
import "./LBToken.sol";
import "./libraries/BinHelper.sol";
import "./libraries/Constants.sol";
import "./libraries/Decoder.sol";
import "./libraries/FeeDistributionHelper.sol";
import "./libraries/Math512Bits.sol";
import "./libraries/Oracle.sol";
import "./libraries/ReentrancyGuardUpgradeable.sol";
import "./libraries/SafeCast.sol";
import "./libraries/SafeMath.sol";
import "./libraries/SwapHelper.sol";
import "./libraries/TokenHelper.sol";
import "./libraries/TreeMath.sol";
import "./interfaces/ILBFlashLoanCallback.sol";
import "./interfaces/ILBPair.sol";

/// @title Liquidity Book Pair
/// @author Trader Joe
/// @notice The implementation of Liquidity Book Pair that also acts as the receipt token for liquidity positions
contract LBPair is LBToken, ReentrancyGuardUpgradeable, ILBPair {
    /** Libraries **/

    using Math512Bits for uint256;
    using TreeMath for mapping(uint256 => uint256)[3];
    using SafeCast for uint256;
    using SafeMath for uint256;
    using TokenHelper for IERC20;
    using FeeHelper for FeeHelper.FeeParameters;
    using SwapHelper for Bin;
    using Decoder for bytes32;
    using FeeDistributionHelper for FeeHelper.FeesDistribution;
    using Oracle for bytes32[65_535];

    /** Modifiers **/

    modifier onlyFactory() {
        if (msg.sender != address(factory)) revert LBPair__OnlyFactory();
        _;
    }

    /** Public immutable variables **/

    ILBFactory public immutable override factory;

    /** Public variables **/

    IERC20 public override tokenX;
    IERC20 public override tokenY;

    /** Private variables **/

    PairInformation private _pairInformation;
    FeeHelper.FeeParameters private _feeParameters;
    /// @dev The reserves of tokens for every bin. This is the amount
    /// of tokenY if `id < _pairInformation.activeId`; of tokenX if `id > _pairInformation.activeId`
    /// and a mix of both if `id == _pairInformation.activeId`
    mapping(uint256 => Bin) private _bins;
    /// @dev Tree to find bins with non zero liquidity
    mapping(uint256 => uint256)[3] private _tree;
    /// @dev Mapping from account to user's unclaimed fees. The first 128 bits are tokenX and the last are for tokenY
    mapping(address => bytes32) private _unclaimedFees;
    /// @dev Mapping from account to id to user's accruedDebt.
    mapping(address => mapping(uint256 => Debts)) private _accruedDebts;
    /// @dev Oracle array
    bytes32[65_535] private _oracle;

    /** OffSets */

    uint256 private constant _OFFSET_PAIR_RESERVE_X = 24;
    uint256 private constant _OFFSET_PROTOCOL_FEE = 128;
    uint256 private constant _OFFSET_BIN_RESERVE_Y = 112;
    uint256 private constant _OFFSET_VARIABLE_FEE_PARAMETERS = 144;
    uint256 private constant _OFFSET_ORACLE_SAMPLE_LIFETIME = 136;
    uint256 private constant _OFFSET_ORACLE_SIZE = 152;
    uint256 private constant _OFFSET_ORACLE_ACTIVE_SIZE = 168;
    uint256 private constant _OFFSET_ORACLE_LAST_TIMESTAMP = 184;
    uint256 private constant _OFFSET_ORACLE_ID = 224;

    /** Constructor **/

    /// @notice Set the factory address
    /// @param _factory The address of the factory
    constructor(ILBFactory _factory) LBToken() {
        if (address(_factory) == address(0)) revert LBPair__AddressZero();
        factory = _factory;
    }

    /// @notice Initialize the parameters of the LBPair
    /// @dev The different parameters needs to be validated very cautiously.
    /// It is highly recommended to never call this function directly, use the factory
    /// as it validates the different parameters
    /// @param _tokenX The address of the tokenX. Can't be address 0
    /// @param _tokenY The address of the tokenY. Can't be address 0
    /// @param _activeId The active id of the pair
    /// @param _sampleLifetime The lifetime of a sample. It's the min time between 2 oracle's sample
    /// @param _packedFeeParameters The fee parameters packed in a single 256 bits slot
    function initialize(
        IERC20 _tokenX,
        IERC20 _tokenY,
        uint24 _activeId,
        uint16 _sampleLifetime,
        bytes32 _packedFeeParameters
    ) external override onlyFactory {
        if (address(_tokenX) == address(0) || address(_tokenY) == address(0)) revert LBPair__AddressZero();
        if (address(tokenX) != address(0)) revert LBPair__AlreadyInitialized();

        __ReentrancyGuard_init();

        tokenX = _tokenX;
        tokenY = _tokenY;

        _pairInformation.activeId = _activeId;
        _pairInformation.oracleSampleLifetime = _sampleLifetime;

        _setFeesParameters(_packedFeeParameters);
        _increaseOracle(2);
    }

    /** External View Functions **/

    /// @notice View function to get the reserves and active id
    /// @return reserveX The reserve of asset X
    /// @return reserveY The reserve of asset Y
    /// @return activeId The active id of the pair
    function getReservesAndId()
        external
        view
        override
        returns (
            uint256 reserveX,
            uint256 reserveY,
            uint256 activeId
        )
    {
        return _getReservesAndId();
    }

    /// @notice View function to get the global fees information, the total fees and those for protocol
    /// @dev The fees for users are `total - protocol`
    /// @return feesXTotal The total fees of asset X
    /// @return feesYTotal The total fees of asset Y
    /// @return feesXProtocol The protocol fees of asset X
    /// @return feesYProtocol The protocol fees of asset Y
    function getGlobalFees()
        external
        view
        override
        returns (
            uint256 feesXTotal,
            uint256 feesYTotal,
            uint256 feesXProtocol,
            uint256 feesYProtocol
        )
    {
        return _getGlobalFees();
    }

    /// @notice View function to get the oracle parameters
    /// @return oracleSampleLifetime The lifetime of a sample, it accumulates information for up to this timestamp
    /// @return oracleSize The size of the oracle (last ids can be empty)
    /// @return oracleActiveSize The active size of the oracle (no empty data)
    /// @return oracleLastTimestamp The timestamp of the creation of the oracle's latest sample
    /// @return oracleId The index of the oracle's latest sample
    /// @return min The min delta time of two samples
    /// @return max The safe max delta time of two samples
    function getOracleParameters()
        external
        view
        override
        returns (
            uint256 oracleSampleLifetime,
            uint256 oracleSize,
            uint256 oracleActiveSize,
            uint256 oracleLastTimestamp,
            uint256 oracleId,
            uint256 min,
            uint256 max
        )
    {
        (oracleSampleLifetime, oracleSize, oracleActiveSize, oracleLastTimestamp, oracleId) = _getOracleParameters();
        min = oracleActiveSize == 0 ? 0 : oracleSampleLifetime;
        max = oracleSampleLifetime * oracleActiveSize;
    }

    /// @notice View function to get the oracle's sample at `_timeDelta` seconds
    /// @dev Return a linearized sample, the weighted average of 2 neighboring samples
    /// @param _timeDelta The number of seconds before the current timestamp
    /// @return cumulativeId The weighted average cumulative id
    /// @return cumulativeVolatilityAccumulated The weighted average cumulative volatility accumulated
    /// @return cumulativeBinCrossed The weighted average cumulative bin crossed
    function getOracleSampleFrom(uint256 _timeDelta)
        external
        view
        override
        returns (
            uint256 cumulativeId,
            uint256 cumulativeVolatilityAccumulated,
            uint256 cumulativeBinCrossed
        )
    {
        uint256 _lookUpTimestamp = block.timestamp - _timeDelta;

        (, , uint256 _oracleActiveSize, , uint256 _oracleId) = _getOracleParameters();

        uint256 timestamp;
        (timestamp, cumulativeId, cumulativeVolatilityAccumulated, cumulativeBinCrossed) = _oracle.getSampleAt(
            _oracleActiveSize,
            _oracleId,
            _lookUpTimestamp
        );

        if (timestamp < _lookUpTimestamp) {
            FeeHelper.FeeParameters memory _fp = _feeParameters;
            uint256 _activeId = _pairInformation.activeId;
            _fp.updateVariableFeeParameters(_activeId);

            unchecked {
                uint256 _deltaT = _lookUpTimestamp - timestamp;

                cumulativeId += _activeId * _deltaT;
                cumulativeVolatilityAccumulated += uint256(_fp.volatilityAccumulated) * _deltaT;
            }
        }
    }

    /// @notice View function to get the fee parameters
    /// @return The fee parameters
    function feeParameters() external view override returns (FeeHelper.FeeParameters memory) {
        return _feeParameters;
    }

    /// @notice View function to get the first bin that isn't empty, will not be `_id` itself
    /// @param _id The bin id
    /// @param _swapForY Whether you've swapping token X for token Y (true) or token Y for token X (false)
    /// @return The id of the non empty bin
    function findFirstNonEmptyBinId(uint24 _id, bool _swapForY) external view override returns (uint24) {
        return _tree.findFirstBin(_id, _swapForY);
    }

    /// @notice View function to get the bin at `id`
    /// @param _id The bin id
    /// @return reserveX The reserve of tokenX of the bin
    /// @return reserveY The reserve of tokenY of the bin
    function getBin(uint24 _id) external view override returns (uint256 reserveX, uint256 reserveY) {
        return _getBin(_id);
    }

    /// @notice View function to get the pending fees of a user
    /// @dev The array must be strictly increasing to ensure uniqueness
    /// @param _account The address of the user
    /// @param _ids The list of ids
    /// @return amountX The amount of tokenX pending
    /// @return amountY The amount of tokenY pending
    function pendingFees(address _account, uint256[] memory _ids)
        external
        view
        override
        returns (uint256 amountX, uint256 amountY)
    {
        bytes32 _unclaimedData = _unclaimedFees[_account];

        amountX = _unclaimedData.decode(type(uint128).max, 0);
        amountY = _unclaimedData.decode(type(uint128).max, 128);

        uint256 _lastId;
        unchecked {
            for (uint256 i; i < _ids.length; ++i) {
                uint256 _id = _ids[i];

                // Ensures uniqueness of ids
                if (_lastId >= _id && i != 0) revert LBPair__OnlyStrictlyIncreasingId();

                uint256 _balance = balanceOf(_account, _id);

                if (_balance != 0) {
                    Bin memory _bin = _bins[_id];

                    (uint256 _amountX, uint256 _amountY) = _getPendingFees(_bin, _account, _id, _balance);

                    amountX += _amountX;
                    amountY += _amountY;
                }

                _lastId = _id;
            }
        }
    }

    /** External Functions **/

    /// @notice Performs a low level swap, this needs to be called from a contract which performs important safety checks
    /// @dev Will swap the full amount that this contract received of token X or Y
    /// @param _swapForY whether the token sent was Y (true) or X (false)
    /// @param _to The address of the recipient
    /// @return amountXOut The amount of token X sent to `_to`
    /// @return amountYOut The amount of token Y sent to `_to`
    function swap(bool _swapForY, address _to)
        external
        override
        nonReentrant
        returns (uint256 amountXOut, uint256 amountYOut)
    {
        PairInformation memory _pair = _pairInformation;

        uint256 _amountIn = _swapForY
            ? tokenX.received(_pair.reserveX, _pair.feesX.total)
            : tokenY.received(_pair.reserveY, _pair.feesY.total);

        if (_amountIn == 0) revert LBPair__InsufficientAmounts();

        FeeHelper.FeeParameters memory _fp = _feeParameters;
        _fp.updateVariableFeeParameters(_pair.activeId);
        uint256 _startId = _pair.activeId;

        uint256 _amountOut;
        // Performs the actual swap, bin per bin
        // It uses the findFirstBin function to make sure the bin we're currently looking at
        // has liquidity in it.
        while (true) {
            Bin memory _bin = _bins[_pair.activeId];
            if ((!_swapForY && _bin.reserveX != 0) || (_swapForY && _bin.reserveY != 0)) {
                (uint256 _amountInToBin, uint256 _amountOutOfBin, FeeHelper.FeesDistribution memory _fees) = _bin
                    .getAmounts(_fp, _pair.activeId, _swapForY, _amountIn);

                _bin.updateFees(_swapForY ? _pair.feesX : _pair.feesY, _fees, _swapForY, totalSupply(_pair.activeId));

                _bin.updateReserves(_pair, _swapForY, _amountInToBin.safe112(), _amountOutOfBin.safe112());

                _amountIn -= _amountInToBin + _fees.total;
                _amountOut += _amountOutOfBin;

                _bins[_pair.activeId] = _bin;

                if (_swapForY) {
                    emit Swap(
                        msg.sender,
                        _to,
                        _pair.activeId,
                        _amountInToBin,
                        0,
                        0,
                        _amountOutOfBin,
                        _fp.volatilityAccumulated,
                        _fees.total,
                        0
                    );
                } else {
                    emit Swap(
                        msg.sender,
                        _to,
                        _pair.activeId,
                        0,
                        _amountInToBin,
                        _amountOutOfBin,
                        0,
                        _fp.volatilityAccumulated,
                        0,
                        _fees.total
                    );
                }
            }

            if (_amountIn != 0) {
                _pair.activeId = _tree.findFirstBin(_pair.activeId, _swapForY);
            } else {
                break;
            }
        }

        if (_amountOut == 0) revert LBPair__BrokenSwapSafetyCheck(); // Safety check

        // We use oracleSize so it can start filling empty slot that were added recently
        uint256 _updatedOracleId = _oracle.update(
            _pair.oracleSize,
            _pair.oracleSampleLifetime,
            _pair.oracleLastTimestamp,
            _pair.oracleId,
            _pair.activeId,
            _fp.volatilityAccumulated,
            _startId.absSub(_pair.activeId)
        );

        // We update the oracleId and lastTimestamp if the sample write on another slot
        if (_updatedOracleId != _pair.oracleId || _pair.oracleLastTimestamp == 0) {
            // Can't overflow as the updatedOracleId < oracleSize
            _pair.oracleId = uint16(_updatedOracleId);
            _pair.oracleLastTimestamp = block.timestamp.safe40();

            // We increase the activeSize if the updated sample is written in a new slot
            // Can't overflow as _updatedOracleId < maxSize = 2**16-1
            unchecked {
                if (_updatedOracleId == _pair.oracleActiveSize) ++_pair.oracleActiveSize;
            }
        }

        _feeParameters = _fp;
        _pairInformation = _pair;

        if (_swapForY) {
            amountYOut = _amountOut;
            tokenY.safeTransfer(_to, _amountOut);
        } else {
            amountXOut = _amountOut;
            tokenX.safeTransfer(_to, _amountOut);
        }
    }

    /// @notice Performs a flash loan
    /// @param _to the address that will execute the external call
    /// @param _amountXOut The amount of tokenX
    /// @param _amountYOut The amount of tokenY
    /// @param _data The bytes data that will be forwarded to _to
    function flashLoan(
        address _to,
        uint256 _amountXOut,
        uint256 _amountYOut,
        bytes calldata _data
    ) external override nonReentrant {
        FeeHelper.FeeParameters memory _fp = _feeParameters;

        uint256 _fee = factory.flashLoanFee();

        FeeHelper.FeesDistribution memory _feesX = _fp.getFeeAmountDistribution(_getFlashLoanFee(_amountXOut, _fee));
        FeeHelper.FeesDistribution memory _feesY = _fp.getFeeAmountDistribution(_getFlashLoanFee(_amountYOut, _fee));

        (uint256 _reserveX, uint256 _reserveY, uint256 _id) = _getReservesAndId();

        tokenX.safeTransfer(_to, _amountXOut);
        tokenY.safeTransfer(_to, _amountYOut);

        ILBFlashLoanCallback(_to).LBFlashLoanCallback(
            msg.sender,
            _amountXOut,
            _amountYOut,
            _feesX.total,
            _feesY.total,
            _data
        );

        _feesX.flashLoanHelper(_pairInformation.feesX, tokenX, _reserveX);
        _feesY.flashLoanHelper(_pairInformation.feesY, tokenY, _reserveY);

        uint256 _totalSupply = totalSupply(_id);

        _bins[_id].accTokenXPerShare += _feesX.getTokenPerShare(_totalSupply);
        _bins[_id].accTokenYPerShare += _feesY.getTokenPerShare(_totalSupply);

        emit FlashLoan(msg.sender, _to, _amountXOut, _amountYOut, _feesX.total, _feesY.total);
    }

    /// @notice Performs a low level add, this needs to be called from a contract which performs important safety checks.
    /// @param _ids The list of ids to add liquidity
    /// @param _distributionX The distribution of tokenX with sum(_distributionX) = 1e18 (100%) or 0 (0%)
    /// @param _distributionY The distribution of tokenY with sum(_distributionY) = 1e18 (100%) or 0 (0%)
    /// @param _to The address of the recipient
    /// @return The amount of token X that was added to the pair
    /// @return The amount of token Y that was added to the pair
    /// @return liquidityMinted Amount of LBToken minted
    function mint(
        uint256[] memory _ids,
        uint256[] memory _distributionX,
        uint256[] memory _distributionY,
        address _to
    )
        external
        override
        nonReentrant
        returns (
            uint256,
            uint256,
            uint256[] memory liquidityMinted
        )
    {
        if (_ids.length == 0 || _ids.length != _distributionX.length || _ids.length != _distributionY.length)
            revert LBPair__WrongLengths();

        PairInformation memory _pair = _pairInformation;

        FeeHelper.FeeParameters memory _fp = _feeParameters;

        MintInfo memory _mintInfo;

        _mintInfo.amountXIn = tokenX.received(_pair.reserveX, _pair.feesX.total).safe128();
        _mintInfo.amountYIn = tokenY.received(_pair.reserveY, _pair.feesY.total).safe128();

        liquidityMinted = new uint256[](_ids.length);

        unchecked {
            for (uint256 i; i < _ids.length; ++i) {
                _mintInfo.id = _ids[i].safe24();
                Bin memory _bin = _bins[_mintInfo.id];

                if (_bin.reserveX == 0 && _bin.reserveY == 0) _tree.addToTree(_mintInfo.id);

                _mintInfo.distributionX = _distributionX[i];
                _mintInfo.distributionY = _distributionY[i];

                if (
                    _mintInfo.distributionX > Constants.PRECISION ||
                    _mintInfo.distributionY > Constants.PRECISION ||
                    (_mintInfo.totalDistributionX += _mintInfo.distributionX) > Constants.PRECISION ||
                    (_mintInfo.totalDistributionY += _mintInfo.distributionY) > Constants.PRECISION
                ) revert LBPair__DistributionsOverflow();

                // Can't overflow as amounts are uint128 and distributions are smaller or equal to 1e18
                _mintInfo.amountX = (_mintInfo.amountXIn * _mintInfo.distributionX) / Constants.PRECISION;
                _mintInfo.amountY = (_mintInfo.amountYIn * _mintInfo.distributionY) / Constants.PRECISION;

                uint256 _price = BinHelper.getPriceFromId(_mintInfo.id, _fp.binStep);
                if (_mintInfo.id >= _pair.activeId) {
                    if (_mintInfo.id == _pair.activeId) {
                        uint256 _totalSupply = totalSupply(_mintInfo.id);

                        uint256 _userL = _price.mulShiftRoundDown(_mintInfo.amountX, Constants.SCALE_OFFSET) +
                            _mintInfo.amountY;

                        uint256 _receivedX;
                        uint256 _receivedY;
                        {
                            uint256 _supply = _totalSupply + _userL;
                            _receivedX = (_userL * (uint256(_bin.reserveX) + _mintInfo.amountX)) / _supply;
                            _receivedY = (_userL * (uint256(_bin.reserveY) + _mintInfo.amountY)) / _supply;
                        }

                        _fp.updateVariableFeeParameters(_mintInfo.id);

                        if (_mintInfo.amountX > _receivedX) {
                            FeeHelper.FeesDistribution memory _fees = _fp.getFeeAmountDistribution(
                                _fp.getFeeAmountForC(_mintInfo.amountX - _receivedX)
                            );

                            _mintInfo.amountX -= _fees.total;
                            _mintInfo.activeFeeX += _fees.total;

                            _bin.updateFees(_pair.feesX, _fees, true, _totalSupply);

                            emit CompositionFee(msg.sender, _to, _mintInfo.id, _fees.total, 0);
                        } else if (_mintInfo.amountY > _receivedY) {
                            FeeHelper.FeesDistribution memory _fees = _fp.getFeeAmountDistribution(
                                _fp.getFeeAmountForC(_mintInfo.amountY - _receivedY)
                            );

                            _mintInfo.amountY -= _fees.total;
                            _mintInfo.activeFeeY += _fees.total;

                            _bin.updateFees(_pair.feesY, _fees, false, _totalSupply);

                            emit CompositionFee(msg.sender, _to, _mintInfo.id, 0, _fees.total);
                        }
                    } else if (_mintInfo.amountY != 0) revert LBPair__CompositionFactorFlawed(_mintInfo.id);
                } else if (_mintInfo.amountX != 0) revert LBPair__CompositionFactorFlawed(_mintInfo.id);

                uint256 _liquidity = _price.mulShiftRoundDown(_mintInfo.amountX, Constants.SCALE_OFFSET) +
                    _mintInfo.amountY;

                if (_liquidity == 0) revert LBPair__InsufficientLiquidityMinted(_mintInfo.id);

                liquidityMinted[i] = _liquidity;

                // The addition can't overflow as the amounts are checked to be uint128 and the reserves are uint112
                _bin.reserveX = (_mintInfo.amountX + _bin.reserveX).safe112();
                _bin.reserveY = (_mintInfo.amountY + _bin.reserveY).safe112();

                // The addition or the cast can't overflow as it would have reverted during the L568 and L569 if amounts were greater than uint112
                _pair.reserveX += uint112(_mintInfo.amountX);
                _pair.reserveY += uint112(_mintInfo.amountY);

                _mintInfo.amountXAddedToPair += _mintInfo.amountX;
                _mintInfo.amountYAddedToPair += _mintInfo.amountY;

                _bins[_mintInfo.id] = _bin;
                _mint(_to, _mintInfo.id, _liquidity);

                emit LiquidityAdded(
                    msg.sender,
                    _to,
                    _mintInfo.id,
                    _liquidity,
                    _mintInfo.amountX,
                    _mintInfo.amountY,
                    _mintInfo.distributionX,
                    _mintInfo.distributionY
                );
            }

            _pairInformation = _pair;

            uint256 _amountAddedPlusFee = _mintInfo.amountXAddedToPair + _mintInfo.activeFeeX;
            // If user sent too much tokens, We send them back the excess
            if (_mintInfo.amountXIn > _amountAddedPlusFee) {
                tokenX.safeTransfer(_to, _mintInfo.amountXIn - _amountAddedPlusFee);
            }

            _amountAddedPlusFee = _mintInfo.amountYAddedToPair + _mintInfo.activeFeeY;
            if (_mintInfo.amountYIn > _amountAddedPlusFee) {
                tokenY.safeTransfer(_to, _mintInfo.amountYIn - _amountAddedPlusFee);
            }
        }

        return (_mintInfo.amountXAddedToPair, _mintInfo.amountYAddedToPair, liquidityMinted);
    }

    /// @notice Performs a low level remove, this needs to be called from a contract which performs important safety checks
    /// @param _ids The ids the user want to remove its liquidity
    /// @param _amounts The amount of token to burn
    /// @param _to The address of the recipient
    /// @return amountX The amount of token X sent to `_to`
    /// @return amountY The amount of token Y sent to `_to`
    function burn(
        uint256[] memory _ids,
        uint256[] memory _amounts,
        address _to
    ) external override nonReentrant returns (uint256 amountX, uint256 amountY) {
        (uint256 _pairReserveX, uint256 _pairReserveY, uint256 _activeId) = _getReservesAndId();
        unchecked {
            for (uint256 i; i < _ids.length; ++i) {
                uint24 _id = _ids[i].safe24();
                uint256 _amountToBurn = _amounts[i];

                if (_amountToBurn == 0) revert LBPair__InsufficientLiquidityBurned(_id);

                (uint256 _reserveX, uint256 _reserveY) = _getBin(_id);

                uint256 _totalSupply = totalSupply(_id);

                uint256 _amountX;
                uint256 _amountY;

                if (_id <= _activeId) {
                    _amountY = _amountToBurn.mulDivRoundDown(_reserveY, _totalSupply);

                    amountY += _amountY;
                    _reserveY -= _amountY;
                    _pairReserveY -= _amountY;
                }
                if (_id >= _activeId) {
                    _amountX = _amountToBurn.mulDivRoundDown(_reserveX, _totalSupply);

                    amountX += _amountX;
                    _reserveX -= _amountX;
                    _pairReserveX -= _amountX;
                }

                if (_reserveX == 0 && _reserveY == 0) _tree.removeFromTree(_id);

                // Optimized `_bins[_id] = _bin` to do only 1 sstore
                assembly {
                    mstore(0, _id)
                    mstore(32, _bins.slot)
                    let slot := keccak256(0, 64)

                    let reserves := add(shl(_OFFSET_BIN_RESERVE_Y, _reserveY), _reserveX)
                    sstore(slot, reserves)
                }

                _burn(address(this), _id, _amountToBurn);

                emit LiquidityRemoved(msg.sender, _to, _id, _amountToBurn, _amountX, _amountY);
            }
        }

        // Optimization to do only 2 sstore
        _pairInformation.reserveX = uint136(_pairReserveX);
        _pairInformation.reserveY = uint136(_pairReserveY);

        tokenX.safeTransfer(_to, amountX);
        tokenY.safeTransfer(_to, amountY);
    }

    /// @notice Increase the length of the oracle
    /// @param _newSize The new size of the oracle. Needs to be bigger than current one
    function increaseOracleLength(uint16 _newSize) external override {
        _increaseOracle(_newSize);
    }

    /// @notice Collect fees of an user
    /// @param _account The address of the user
    /// @param _ids The list of bin ids to collect fees in
    /// @return amountX The amount of tokenX claimed
    /// @return amountY The amount of tokenY claimed
    function collectFees(address _account, uint256[] memory _ids)
        external
        override
        nonReentrant
        returns (uint256 amountX, uint256 amountY)
    {
        unchecked {
            bytes32 _unclaimedData = _unclaimedFees[_account];
            delete _unclaimedFees[_account];

            amountX = _unclaimedData.decode(type(uint128).max, 0);
            amountY = _unclaimedData.decode(type(uint128).max, 128);

            for (uint256 i; i < _ids.length; ++i) {
                uint256 _id = _ids[i];
                uint256 _balance = balanceOf(_account, _id);

                if (_balance != 0) {
                    Bin memory _bin = _bins[_id];

                    (uint256 _amountX, uint256 _amountY) = _getPendingFees(_bin, _account, _id, _balance);
                    _updateUserDebts(_bin, _account, _id, _balance);

                    amountX += _amountX;
                    amountY += _amountY;
                }
            }

            if (amountX != 0) {
                _pairInformation.feesX.total -= uint128(amountX);
            }
            if (amountY != 0) {
                _pairInformation.feesY.total -= uint128(amountY);
            }

            tokenX.safeTransfer(_account, amountX);
            tokenY.safeTransfer(_account, amountY);

            emit FeesCollected(msg.sender, _account, amountX, amountY);
        }
    }

    /// @notice Collect the protocol fees and send them to the feeRecipient
    /// @dev The balances are not zeroed to save gas by not resetting the storage slot
    /// Only callable by the fee recipient
    /// @return amountX The amount of tokenX claimed
    /// @return amountY The amount of tokenY claimed
    function collectProtocolFees() external override nonReentrant returns (uint256 amountX, uint256 amountY) {
        unchecked {
            address _feeRecipient = factory.feeRecipient();

            if (msg.sender != _feeRecipient) revert LBPair__OnlyFeeRecipient(_feeRecipient, msg.sender);

            // The fees returned can't be greater than uint128, so the assembly blocks are safe
            (
                uint256 _feesXTotal,
                uint256 _feesYTotal,
                uint256 _feesXProtocol,
                uint256 _feesYProtocol
            ) = _getGlobalFees();

            if (_feesXProtocol > 1) {
                amountX = _feesXProtocol - 1;
                _feesXTotal -= amountX;

                // Assembly block that does:
                // _pairInformation.feesX = FeeHelper.FeesDistribution({total: _feesXTotal, protocol: 1});
                assembly {
                    let _slotX := add(_pairInformation.slot, 2)

                    sstore(_slotX, add(shl(_OFFSET_PROTOCOL_FEE, 1), _feesXTotal))
                }

                tokenX.safeTransfer(_feeRecipient, amountX);
            }

            if (_feesYProtocol > 1) {
                amountY = _feesYProtocol - 1;
                _feesYTotal -= amountY;

                // Assembly block that does:
                // _pairInformation.feesY = FeeHelper.FeesDistribution({total: _feesYTotal, protocol: 1});
                assembly {
                    let _slotY := add(_pairInformation.slot, 3)

                    sstore(_slotY, add(shl(_OFFSET_PROTOCOL_FEE, 1), _feesYTotal))
                }

                tokenY.safeTransfer(_feeRecipient, amountY);
            }

            emit ProtocolFeesCollected(msg.sender, _feeRecipient, amountX, amountY);
        }
    }

    /// @notice Set the fees parameters
    /// @dev Needs to be called by the factory that will validate the values
    /// The bin step will not change
    /// Only callable by the factory
    /// @param _packedFeeParameters The packed fee parameters
    function setFeesParameters(bytes32 _packedFeeParameters) external override onlyFactory {
        _setFeesParameters(_packedFeeParameters);
    }

    function forceDecay() external override onlyFactory {
        unchecked {
            _feeParameters.volatilityReference = uint24(
                (uint256(_feeParameters.reductionFactor) * _feeParameters.volatilityReference) /
                    Constants.BASIS_POINT_MAX
            );
        }
    }

    /** Internal Functions **/

    /// @notice Collect and update fees before any token transfer, mint or burn
    /// @param _from The address of the owner of the token
    /// @param _to The address of the recipient of the  token
    /// @param _id The id of the token
    /// @param _amount The amount of token of type `id`
    function _beforeTokenTransfer(
        address _from,
        address _to,
        uint256 _id,
        uint256 _amount
    ) internal override(LBToken) {
        unchecked {
            super._beforeTokenTransfer(_from, _to, _id, _amount);

            Bin memory _bin = _bins[_id];

            if (_from != _to) {
                if (_from != address(0) && _from != address(this)) {
                    uint256 _balanceFrom = balanceOf(_from, _id);

                    _cacheFees(_bin, _from, _id, _balanceFrom, _balanceFrom - _amount);
                }

                if (_to != address(0) && _to != address(this)) {
                    uint256 _balanceTo = balanceOf(_to, _id);

                    _cacheFees(_bin, _to, _id, _balanceTo, _balanceTo + _amount);
                }
            }
        }
    }

    /** Private Functions **/

    /// @notice View function to get the pending fees of an account on a given bin
    /// @param _bin  The bin where the user is collecting fees
    /// @param _account The address of the user
    /// @param _id The id where the user is collecting fees
    /// @param _balance The previous balance of the user
    /// @return amountX The amount of tokenX pending for the account
    /// @return amountY The amount of tokenY pending for the account
    function _getPendingFees(
        Bin memory _bin,
        address _account,
        uint256 _id,
        uint256 _balance
    ) private view returns (uint256 amountX, uint256 amountY) {
        Debts memory _debts = _accruedDebts[_account][_id];

        amountX = _bin.accTokenXPerShare.mulShiftRoundDown(_balance, Constants.SCALE_OFFSET) - _debts.debtX;
        amountY = _bin.accTokenYPerShare.mulShiftRoundDown(_balance, Constants.SCALE_OFFSET) - _debts.debtY;
    }

    /// @notice Update fees of a given user
    /// @param _bin The bin where the user has collected fees
    /// @param _account The address of the user
    /// @param _id The id where the user has collected fees
    /// @param _balance The new balance of the user
    function _updateUserDebts(
        Bin memory _bin,
        address _account,
        uint256 _id,
        uint256 _balance
    ) private {
        uint256 _debtX = _bin.accTokenXPerShare.mulShiftRoundDown(_balance, Constants.SCALE_OFFSET);
        uint256 _debtY = _bin.accTokenYPerShare.mulShiftRoundDown(_balance, Constants.SCALE_OFFSET);

        _accruedDebts[_account][_id].debtX = _debtX;
        _accruedDebts[_account][_id].debtY = _debtY;
    }

    /// @notice Update the unclaimed fees of a given user before a transfer
    /// @param _bin The bin where the user has collected fees
    /// @param _user The address of the user
    /// @param _id The id where the user has collected fees
    /// @param _previousBalance The previous balance of the user
    /// @param _newBalance The new balance of the user
    function _cacheFees(
        Bin memory _bin,
        address _user,
        uint256 _id,
        uint256 _previousBalance,
        uint256 _newBalance
    ) private {
        unchecked {
            bytes32 _unclaimedData = _unclaimedFees[_user];

            uint256 amountX = _unclaimedData.decode(type(uint128).max, 0);
            uint256 amountY = _unclaimedData.decode(type(uint128).max, 128);

            (uint256 _amountX, uint256 _amountY) = _getPendingFees(_bin, _user, _id, _previousBalance);
            _updateUserDebts(_bin, _user, _id, _newBalance);

            (amountX += _amountX).safe128();
            (amountY += _amountY).safe128();

            _unclaimedFees[_user] = bytes32((amountY << 128) | amountX);
        }
    }

    /// @notice Internal function to set the fee parameters of the pair
    /// @param _packedFeeParameters The packed fee parameters
    function _setFeesParameters(bytes32 _packedFeeParameters) internal {
        bytes32 _feeStorageSlot;
        assembly {
            _feeStorageSlot := sload(_feeParameters.slot)
        }

        uint256 _varParameters = _feeStorageSlot.decode(type(uint112).max, _OFFSET_VARIABLE_FEE_PARAMETERS);
        uint256 _newFeeParameters = _packedFeeParameters.decode(type(uint144).max, 0);

        assembly {
            sstore(_feeParameters.slot, or(_newFeeParameters, _varParameters))
        }
    }

    /// @notice Private function to increase the oracle's number of sample
    /// @param _newSize The new size of the oracle. Needs to be bigger than current one
    function _increaseOracle(uint16 _newSize) private {
        uint256 _oracleSize = _pairInformation.oracleSize;

        if (_oracleSize >= _newSize) revert LBPair__NewSizeTooSmall(_newSize, _oracleSize);

        _pairInformation.oracleSize = _newSize;

        unchecked {
            for (uint256 _id = _oracleSize; _id < _newSize; ++_id) {
                _oracle.initialize(_id);
            }
        }

        emit OracleSizeIncreased(_oracleSize, _newSize);
    }

    /// @notice Private view function to return the oracle's parameters
    /// @return oracleSampleLifetime The lifetime of a sample, it accumulates information for up to this timestamp
    /// @return oracleSize The size of the oracle (last ids can be empty)
    /// @return oracleActiveSize The active size of the oracle (no empty data)
    /// @return oracleLastTimestamp The timestamp of the creation of the oracle's latest sample
    /// @return oracleId The index of the oracle's latest sample
    function _getOracleParameters()
        internal
        view
        returns (
            uint256 oracleSampleLifetime,
            uint256 oracleSize,
            uint256 oracleActiveSize,
            uint256 oracleLastTimestamp,
            uint256 oracleId
        )
    {
        bytes32 _slot;
        assembly {
            _slot := sload(add(_pairInformation.slot, 1))
        }
        oracleSampleLifetime = _slot.decode(type(uint16).max, _OFFSET_ORACLE_SAMPLE_LIFETIME);
        oracleSize = _slot.decode(type(uint16).max, _OFFSET_ORACLE_SIZE);
        oracleActiveSize = _slot.decode(type(uint16).max, _OFFSET_ORACLE_ACTIVE_SIZE);
        oracleLastTimestamp = _slot.decode(type(uint40).max, _OFFSET_ORACLE_LAST_TIMESTAMP);
        oracleId = _slot.decode(type(uint24).max, _OFFSET_ORACLE_ID);
    }

    /// @notice Internal view function to get the reserves and active id
    /// @return reserveX The reserve of asset X
    /// @return reserveY The reserve of asset Y
    /// @return activeId The active id of the pair
    function _getReservesAndId()
        internal
        view
        returns (
            uint256 reserveX,
            uint256 reserveY,
            uint256 activeId
        )
    {
        uint256 _mask24 = type(uint24).max;
        uint256 _mask136 = type(uint136).max;
        assembly {
            let slot := sload(add(_pairInformation.slot, 1))
            reserveY := and(slot, _mask136)

            slot := sload(_pairInformation.slot)
            activeId := and(slot, _mask24)
            reserveX := and(shr(_OFFSET_PAIR_RESERVE_X, slot), _mask136)
        }
    }

    /// @notice Internal view function to get the bin at `id`
    /// @param _id The bin id
    /// @return reserveX The reserve of tokenX of the bin
    /// @return reserveY The reserve of tokenY of the bin
    function _getBin(uint24 _id) internal view returns (uint256 reserveX, uint256 reserveY) {
        bytes32 _data;
        uint256 _mask112 = type(uint112).max;
        // low level read of mapping to only load 1 storage slot
        assembly {
            mstore(0, _id)
            mstore(32, _bins.slot)
            _data := sload(keccak256(0, 64))

            reserveX := and(_data, _mask112)
            reserveY := shr(_OFFSET_BIN_RESERVE_Y, _data)
        }

        return (reserveX.safe112(), reserveY.safe112());
    }

    /// @notice Internal view function to get the global fees information, the total fees and those for protocol
    /// @dev The fees for users are `total - protocol`
    /// @return feesXTotal The total fees of asset X
    /// @return feesYTotal The total fees of asset Y
    /// @return feesXProtocol The protocol fees of asset X
    /// @return feesYProtocol The protocol fees of asset Y
    function _getGlobalFees()
        internal
        view
        returns (
            uint256 feesXTotal,
            uint256 feesYTotal,
            uint256 feesXProtocol,
            uint256 feesYProtocol
        )
    {
        bytes32 _slotX;
        bytes32 _slotY;
        assembly {
            _slotX := sload(add(_pairInformation.slot, 2))
            _slotY := sload(add(_pairInformation.slot, 3))
        }

        feesXTotal = _slotX.decode(type(uint128).max, 0);
        feesYTotal = _slotY.decode(type(uint128).max, 0);

        feesXProtocol = _slotX.decode(type(uint128).max, _OFFSET_PROTOCOL_FEE);
        feesYProtocol = _slotY.decode(type(uint128).max, _OFFSET_PROTOCOL_FEE);
    }

    /// @notice Internal pure function to return the flashloan fee amount
    /// @param _amount The amount to flashloan
    /// @param _fee the fee percentage, in basis point
    /// @return The fee amount
    function _getFlashLoanFee(uint256 _amount, uint256 _fee) internal pure returns (uint256) {
        return (_amount * _fee) / Constants.PRECISION;
    }
}
