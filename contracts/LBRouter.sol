// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "openzeppelin/token/ERC20/IERC20.sol";

import "./LBErrors.sol";
import "./libraries/BinHelper.sol";
import "./libraries/Constants.sol";
import "./libraries/FeeHelper.sol";
import "./libraries/Math512Bits.sol";
import "./libraries/SwapHelper.sol";
import "./libraries/TokenHelper.sol";
import "./interfaces/IJoePair.sol";
import "./interfaces/ILBToken.sol";
import "./interfaces/ILBRouter.sol";

/// @title Liquidity Book Router
/// @author Trader Joe
/// @notice Main contract to interact with to swap and manage liquidity on Joe V2 exchange.
contract LBRouter is ILBRouter {
    using TokenHelper for IERC20;
    using TokenHelper for IWAVAX;
    using FeeHelper for FeeHelper.FeeParameters;
    using Math512Bits for uint256;
    using SwapHelper for ILBPair.Bin;

    ILBFactory public immutable override factory;
    IJoeFactory public immutable override oldFactory;
    IWAVAX public immutable override wavax;

    modifier onlyFactoryOwner() {
        if (msg.sender != factory.owner()) revert LBRouter__NotFactoryOwner();
        _;
    }

    modifier ensure(uint256 _deadline) {
        if (block.timestamp > _deadline) revert LBRouter__DeadlineExceeded(_deadline, block.timestamp);
        _;
    }

    modifier verifyInputs(uint256[] memory _pairBinSteps, IERC20[] memory _tokenPath) {
        if (_pairBinSteps.length == 0 || _pairBinSteps.length + 1 != _tokenPath.length)
            revert LBRouter__LengthsMismatch();
        _;
    }

    /// @notice Constructor
    /// @param _factory LBFactory address
    /// @param _oldFactory Address of old factory (Joe V1)
    /// @param _wavax Address of WAVAX
    constructor(
        ILBFactory _factory,
        IJoeFactory _oldFactory,
        IWAVAX _wavax
    ) {
        factory = _factory;
        oldFactory = _oldFactory;
        wavax = _wavax;
    }

    /// @dev Receive function that only accept AVAX from the WAVAX contract
    receive() external payable {
        if (msg.sender != address(wavax)) revert LBRouter__SenderIsNotWAVAX();
    }

    /// @notice Returns the approximate id corresponding to the inputted price.
    /// Warning, the returned id may be inaccurate close to the start price of a bin
    /// @param _LBPair The address of the LBPair
    /// @param _price The price of y per x (multiplied by 1e36)
    /// @return The id corresponding to this price
    function getIdFromPrice(ILBPair _LBPair, uint256 _price) external view override returns (uint24) {
        return BinHelper.getIdFromPrice(_price, _LBPair.feeParameters().binStep);
    }

    /// @notice Returns the price corresponding to the inputted id
    /// @param _LBPair The address of the LBPair
    /// @param _id The id
    /// @return The price corresponding to this id
    function getPriceFromId(ILBPair _LBPair, uint24 _id) external view override returns (uint256) {
        return BinHelper.getPriceFromId(_id, _LBPair.feeParameters().binStep);
    }

    /// @notice Simulate a swap in
    /// @param _LBPair The address of the LBPair
    /// @param _amountOut The amount of token to receive
    /// @param _swapForY Whether you swap X for Y (true), or Y for X (false)
    /// @return amountIn The amount of token to send in order to receive _amountOut token
    /// @return feesIn The amount of fees paid in token sent
    function getSwapIn(
        ILBPair _LBPair,
        uint256 _amountOut,
        bool _swapForY
    ) public view override returns (uint256 amountIn, uint256 feesIn) {
        (uint256 _pairReserveX, uint256 _pairReserveY, uint256 _activeId) = _LBPair.getReservesAndId();

        if (_amountOut == 0 || (_swapForY ? _amountOut > _pairReserveY : _amountOut > _pairReserveX))
            revert LBRouter__WrongAmounts(_amountOut, _swapForY ? _pairReserveY : _pairReserveX); // If this is wrong, then we're sure the amounts sent are wrong

        FeeHelper.FeeParameters memory _fp = _LBPair.feeParameters();
        _fp.updateVariableFeeParameters(_activeId);

        uint256 _amountOutOfBin;
        uint256 _amountInWithFees;
        uint256 _reserve;
        // Performs the actual swap, bin per bin
        // It uses the findFirstNonEmptyBinId function to make sure the bin we're currently looking at
        // has liquidity in it.
        while (true) {
            {
                (uint256 _reserveX, uint256 _reserveY) = _LBPair.getBin(uint24(_activeId));
                _reserve = _swapForY ? _reserveY : _reserveX;
            }
            uint256 _price = BinHelper.getPriceFromId(_activeId, _fp.binStep);
            if (_reserve != 0) {
                _amountOutOfBin = _amountOut > _reserve ? _reserve : _amountOut;

                uint256 _amountInToBin = _swapForY
                    ? _amountOutOfBin.shiftDivRoundUp(Constants.SCALE_OFFSET, _price)
                    : _price.mulShiftRoundUp(_amountOutOfBin, Constants.SCALE_OFFSET);

                // We update the fee, but we don't store the new volatility reference, volatility accumulated and indexRef to not penalize traders
                _fp.updateVolatilityAccumulated(_activeId);
                uint256 _fee = _fp.getFeeAmount(_amountInToBin);
                _amountInWithFees = _amountInToBin + _fee;

                if (_amountInWithFees + _reserve > type(uint112).max) revert LBRouter__SwapOverflows(_activeId);
                amountIn += _amountInWithFees;
                feesIn += _fee;
                _amountOut -= _amountOutOfBin;
            }

            if (_amountOut != 0) {
                _activeId = _LBPair.findFirstNonEmptyBinId(uint24(_activeId), _swapForY);
            } else {
                break;
            }
        }
        if (_amountOut != 0) revert LBRouter__BrokenSwapSafetyCheck(); // Safety check, but should never be false as it would have reverted on transfer
    }

    /// @notice Simulate a swap out
    /// @param _LBPair The address of the LBPair
    /// @param _amountIn The amount of token sent
    /// @param _swapForY Whether you swap X for Y (true), or Y for X (false)
    /// @return amountOut The amount of token received if _amountIn tokenX are sent
    /// @return feesIn The amount of fees paid in token sent
    function getSwapOut(
        ILBPair _LBPair,
        uint256 _amountIn,
        bool _swapForY
    ) external view override returns (uint256 amountOut, uint256 feesIn) {
        (, , uint256 _activeId) = _LBPair.getReservesAndId();

        FeeHelper.FeeParameters memory _fp = _LBPair.feeParameters();
        _fp.updateVariableFeeParameters(_activeId);
        ILBPair.Bin memory _bin;

        // Performs the actual swap, bin per bin
        // It uses the findFirstNonEmptyBinId function to make sure the bin we're currently looking at
        // has liquidity in it.
        while (true) {
            {
                (uint256 _reserveX, uint256 _reserveY) = _LBPair.getBin(uint24(_activeId));
                _bin = ILBPair.Bin(uint112(_reserveX), uint112(_reserveY), 0, 0);
            }
            if (_bin.reserveX != 0 || _bin.reserveY != 0) {
                (uint256 _amountInToBin, uint256 _amountOutOfBin, FeeHelper.FeesDistribution memory _fees) = _bin
                    .getAmounts(_fp, _activeId, _swapForY, _amountIn);

                if (_amountInToBin > type(uint112).max) revert LBRouter__BinReserveOverflows(_activeId);

                _amountIn -= _amountInToBin + _fees.total;
                feesIn += _fees.total;
                amountOut += _amountOutOfBin;
            }

            if (_amountIn != 0) {
                _activeId = _LBPair.findFirstNonEmptyBinId(uint24(_activeId), _swapForY);
            } else {
                break;
            }
        }
        if (_amountIn != 0) revert LBRouter__TooMuchTokensIn(_amountIn);
    }

    /// @notice Create a liquidity bin LBPair for _tokenX and _tokenY using the factory
    /// @param _tokenX The address of the first token
    /// @param _tokenY The address of the second token
    /// @param _activeId The active id of the pair
    /// @param _binStep The bin step in basis point, used to calculate log(1 + binStep)
    /// @return pair The address of the newly created LBPair
    function createLBPair(
        IERC20 _tokenX,
        IERC20 _tokenY,
        uint24 _activeId,
        uint16 _binStep
    ) external override returns (ILBPair pair) {
        pair = factory.createLBPair(_tokenX, _tokenY, _activeId, _binStep);
    }

    /// @notice Add liquidity while performing safety checks
    /// @dev This function is compliant with fee on transfer tokens
    /// @param _liquidityParameters The liquidity parameters
    /// @return depositIds Bin ids where the liquidity was actually deposited
    /// @return liquidityMinted Amounts of LBToken minted for each bin
    function addLiquidity(LiquidityParameters memory _liquidityParameters)
        external
        override
        returns (uint256[] memory depositIds, uint256[] memory liquidityMinted)
    {
        ILBPair _LBPair = _getLBPairInformation(
            _liquidityParameters.tokenX,
            _liquidityParameters.tokenY,
            _liquidityParameters.binStep
        );
        if (_liquidityParameters.tokenX != _LBPair.tokenX()) revert LBRouter__WrongTokenOrder();

        _liquidityParameters.tokenX.safeTransferFrom(msg.sender, address(_LBPair), _liquidityParameters.amountX);
        _liquidityParameters.tokenY.safeTransferFrom(msg.sender, address(_LBPair), _liquidityParameters.amountY);

        (depositIds, liquidityMinted) = _addLiquidity(_liquidityParameters, _LBPair);
    }

    /// @notice Add liquidity with AVAX while performing safety checks
    /// @dev This function is compliant with fee on transfer tokens
    /// @param _liquidityParameters The liquidity parameters
    /// @return depositIds Bin ids where the liquidity was actually deposited
    /// @return liquidityMinted Amounts of LBToken minted for each bin
    function addLiquidityAVAX(LiquidityParameters memory _liquidityParameters)
        external
        payable
        override
        returns (uint256[] memory depositIds, uint256[] memory liquidityMinted)
    {
        ILBPair _LBPair = _getLBPairInformation(
            _liquidityParameters.tokenX,
            _liquidityParameters.tokenY,
            _liquidityParameters.binStep
        );
        if (_liquidityParameters.tokenX != _LBPair.tokenX()) revert LBRouter__WrongTokenOrder();

        if (_liquidityParameters.tokenX == wavax && _liquidityParameters.amountX == msg.value) {
            _wavaxDepositAndTransfer(address(_LBPair), msg.value);
            _liquidityParameters.tokenY.safeTransferFrom(msg.sender, address(_LBPair), _liquidityParameters.amountY);
        } else if (_liquidityParameters.tokenY == wavax && _liquidityParameters.amountY == msg.value) {
            _liquidityParameters.tokenX.safeTransferFrom(msg.sender, address(_LBPair), _liquidityParameters.amountX);
            _wavaxDepositAndTransfer(address(_LBPair), msg.value);
        } else
            revert LBRouter__WrongAvaxLiquidityParameters(
                address(_liquidityParameters.tokenX),
                address(_liquidityParameters.tokenY),
                _liquidityParameters.amountX,
                _liquidityParameters.amountY,
                msg.value
            );

        (depositIds, liquidityMinted) = _addLiquidity(_liquidityParameters, _LBPair);
    }

    /// @notice Remove liquidity while performing safety checks
    /// @dev This function is compliant with fee on transfer tokens
    /// @param _tokenX The address of token X
    /// @param _tokenY The address of token Y
    /// @param _binStep The bin step of the LBPair
    /// @param _amountXMin The min amount to receive of token X
    /// @param _amountYMin The min amount to receive of token Y
    /// @param _ids The list of ids to burn
    /// @param _amounts The list of amounts to burn of each id in `_ids`
    /// @param _to The address of the recipient
    /// @param _deadline The deadline of the tx
    /// @return amountX Amount of token X returned
    /// @return amountY Amount of token Y returned
    function removeLiquidity(
        IERC20 _tokenX,
        IERC20 _tokenY,
        uint16 _binStep,
        uint256 _amountXMin,
        uint256 _amountYMin,
        uint256[] memory _ids,
        uint256[] memory _amounts,
        address _to,
        uint256 _deadline
    ) external override ensure(_deadline) returns (uint256 amountX, uint256 amountY) {
        ILBPair _LBPair = _getLBPairInformation(_tokenX, _tokenY, _binStep);
        if (_tokenX != _LBPair.tokenX()) {
            (_tokenX, _tokenY) = (_tokenY, _tokenX);
            (_amountXMin, _amountYMin) = (_amountYMin, _amountXMin);
        }

        (amountX, amountY) = _removeLiquidity(_LBPair, _amountXMin, _amountYMin, _ids, _amounts, _to);
    }

    /// @notice Remove AVAX liquidity while performing safety checks
    /// @dev This function is **NOT** compliant with fee on transfer tokens.
    /// This is wanted as it would make users pays the fee on transfer twice,
    /// use the `removeLiquidity` function to remove liquidity with fee on transfer tokens.
    /// @param _token The address of token
    /// @param _binStep The bin step of the LBPair
    /// @param _amountTokenMin The min amount to receive of token
    /// @param _amountAVAXMin The min amount to receive of AVAX
    /// @param _ids The list of ids to burn
    /// @param _amounts The list of amounts to burn of each id in `_ids`
    /// @param _to The address of the recipient
    /// @param _deadline The deadline of the tx
    /// @return amountToken Amount of token returned
    /// @return amountAVAX Amount of AVAX returned
    function removeLiquidityAVAX(
        IERC20 _token,
        uint16 _binStep,
        uint256 _amountTokenMin,
        uint256 _amountAVAXMin,
        uint256[] memory _ids,
        uint256[] memory _amounts,
        address payable _to,
        uint256 _deadline
    ) external override ensure(_deadline) returns (uint256 amountToken, uint256 amountAVAX) {
        ILBPair _LBPair = _getLBPairInformation(_token, IERC20(wavax), _binStep);

        bool _isAVAXTokenY = IERC20(wavax) == _LBPair.tokenY();
        {
            if (!_isAVAXTokenY) {
                (_amountTokenMin, _amountAVAXMin) = (_amountAVAXMin, _amountTokenMin);
            }

            (uint256 _amountX, uint256 _amountY) = _removeLiquidity(
                _LBPair,
                _amountTokenMin,
                _amountAVAXMin,
                _ids,
                _amounts,
                address(this)
            );

            (amountToken, amountAVAX) = _isAVAXTokenY ? (_amountX, _amountY) : (_amountY, _amountX);
        }

        _token.safeTransfer(_to, amountToken);

        wavax.withdraw(amountAVAX);
        _safeTransferAVAX(_to, amountAVAX);
    }

    /// @notice Swaps exact tokens for tokens while performing safety checks
    /// @param _amountIn The amount of token to send
    /// @param _amountOutMin The min amount of token to receive
    /// @param _pairBinSteps The bin step of the pairs (0: V1, other values will use V2)
    /// @param _tokenPath The swap path using the binSteps following `_pairBinSteps`
    /// @param _to The address of the recipient
    /// @param _deadline The deadline of the tx
    /// @return amountOut Output amount of the swap
    function swapExactTokensForTokens(
        uint256 _amountIn,
        uint256 _amountOutMin,
        uint256[] memory _pairBinSteps,
        IERC20[] memory _tokenPath,
        address _to,
        uint256 _deadline
    ) external override ensure(_deadline) verifyInputs(_pairBinSteps, _tokenPath) returns (uint256 amountOut) {
        address[] memory _pairs = _getPairs(_pairBinSteps, _tokenPath);

        _tokenPath[0].safeTransferFrom(msg.sender, _pairs[0], _amountIn);

        amountOut = _swapExactTokensForTokens(_amountIn, _pairs, _pairBinSteps, _tokenPath, _to);

        if (_amountOutMin > amountOut) revert LBRouter__InsufficientAmountOut(_amountOutMin, amountOut);
    }

    /// @notice Swaps exact tokens for AVAX while performing safety checks
    /// @param _amountIn The amount of token to send
    /// @param _amountOutMinAVAX The min amount of AVAX to receive
    /// @param _pairBinSteps The bin step of the pairs (0: V1, other values will use V2)
    /// @param _tokenPath The swap path using the binSteps following `_pairBinSteps`
    /// @param _to The address of the recipient
    /// @param _deadline The deadline of the tx
    /// @return amountOut Output amount of the swap
    function swapExactTokensForAVAX(
        uint256 _amountIn,
        uint256 _amountOutMinAVAX,
        uint256[] memory _pairBinSteps,
        IERC20[] memory _tokenPath,
        address payable _to,
        uint256 _deadline
    ) external override ensure(_deadline) verifyInputs(_pairBinSteps, _tokenPath) returns (uint256 amountOut) {
        if (_tokenPath[_pairBinSteps.length] != IERC20(wavax))
            revert LBRouter__InvalidTokenPath(address(_tokenPath[_pairBinSteps.length]));

        address[] memory _pairs = _getPairs(_pairBinSteps, _tokenPath);

        _tokenPath[0].safeTransferFrom(msg.sender, _pairs[0], _amountIn);

        amountOut = _swapExactTokensForTokens(_amountIn, _pairs, _pairBinSteps, _tokenPath, address(this));

        if (_amountOutMinAVAX > amountOut) revert LBRouter__InsufficientAmountOut(_amountOutMinAVAX, amountOut);

        wavax.withdraw(amountOut);
        _safeTransferAVAX(_to, amountOut);
    }

    /// @notice Swaps exact AVAX for tokens while performing safety checks
    /// @param _amountOutMin The min amount of token to receive
    /// @param _pairBinSteps The bin step of the pairs (0: V1, other values will use V2)
    /// @param _tokenPath The swap path using the binSteps following `_pairBinSteps`
    /// @param _to The address of the recipient
    /// @param _deadline The deadline of the tx
    /// @return amountOut Output amount of the swap
    function swapExactAVAXForTokens(
        uint256 _amountOutMin,
        uint256[] memory _pairBinSteps,
        IERC20[] memory _tokenPath,
        address _to,
        uint256 _deadline
    ) external payable override ensure(_deadline) verifyInputs(_pairBinSteps, _tokenPath) returns (uint256 amountOut) {
        address[] memory _pairs = _getPairs(_pairBinSteps, _tokenPath);

        _wavaxDepositAndTransfer(_pairs[0], msg.value);

        amountOut = _swapExactTokensForTokens(msg.value, _pairs, _pairBinSteps, _tokenPath, _to);

        if (_amountOutMin > amountOut) revert LBRouter__InsufficientAmountOut(_amountOutMin, amountOut);
    }

    /// @notice Swaps tokens for exact tokens while performing safety checks
    /// @param _amountOut The amount of token to receive
    /// @param _amountInMax The max amount of token to send
    /// @param _pairBinSteps The bin step of the pairs (0: V1, other values will use V2)
    /// @param _tokenPath The swap path using the binSteps following `_pairBinSteps`
    /// @param _to The address of the recipient
    /// @param _deadline The deadline of the tx
    /// @return amountsIn Input amounts for every step of the swap
    function swapTokensForExactTokens(
        uint256 _amountOut,
        uint256 _amountInMax,
        uint256[] memory _pairBinSteps,
        IERC20[] memory _tokenPath,
        address _to,
        uint256 _deadline
    ) external override ensure(_deadline) verifyInputs(_pairBinSteps, _tokenPath) returns (uint256[] memory amountsIn) {
        address[] memory _pairs = _getPairs(_pairBinSteps, _tokenPath);
        amountsIn = _getAmountsIn(_pairBinSteps, _pairs, _tokenPath, _amountOut);

        if (amountsIn[0] > _amountInMax) revert LBRouter__MaxAmountInExceeded(_amountInMax, amountsIn[0]);

        _tokenPath[0].safeTransferFrom(msg.sender, _pairs[0], amountsIn[0]);

        uint256 _amountOutReal = _swapTokensForExactTokens(_pairs, _pairBinSteps, _tokenPath, amountsIn, _to);

        if (_amountOutReal < _amountOut) revert LBRouter__InsufficientAmountOut(_amountOut, _amountOutReal);
    }

    /// @notice Swaps tokens for exact AVAX while performing safety checks
    /// @param _amountAVAXOut The amount of AVAX to receive
    /// @param _amountInMax The max amount of token to send
    /// @param _pairBinSteps The bin step of the pairs (0: V1, other values will use V2)
    /// @param _tokenPath The swap path using the binSteps following `_pairBinSteps`
    /// @param _to The address of the recipient
    /// @param _deadline The deadline of the tx
    /// @return amountsIn Input amounts for every step of the swap
    function swapTokensForExactAVAX(
        uint256 _amountAVAXOut,
        uint256 _amountInMax,
        uint256[] memory _pairBinSteps,
        IERC20[] memory _tokenPath,
        address payable _to,
        uint256 _deadline
    ) external override ensure(_deadline) verifyInputs(_pairBinSteps, _tokenPath) returns (uint256[] memory amountsIn) {
        if (_tokenPath[_pairBinSteps.length] != IERC20(wavax))
            revert LBRouter__InvalidTokenPath(address(_tokenPath[_pairBinSteps.length]));

        address[] memory _pairs = _getPairs(_pairBinSteps, _tokenPath);
        amountsIn = _getAmountsIn(_pairBinSteps, _pairs, _tokenPath, _amountAVAXOut);

        if (amountsIn[0] > _amountInMax) revert LBRouter__MaxAmountInExceeded(_amountInMax, amountsIn[0]);

        _tokenPath[0].safeTransferFrom(msg.sender, _pairs[0], amountsIn[0]);

        uint256 _amountOutReal = _swapTokensForExactTokens(_pairs, _pairBinSteps, _tokenPath, amountsIn, address(this));

        if (_amountOutReal < _amountAVAXOut) revert LBRouter__InsufficientAmountOut(_amountAVAXOut, _amountOutReal);

        wavax.withdraw(_amountOutReal);
        _safeTransferAVAX(_to, _amountOutReal);
    }

    /// @notice Swaps AVAX for exact tokens while performing safety checks
    /// @dev will refund any excess sent
    /// @param _amountOut The amount of tokens to receive
    /// @param _pairBinSteps The bin step of the pairs (0: V1, other values will use V2)
    /// @param _tokenPath The swap path using the binSteps following `_pairBinSteps`
    /// @param _to The address of the recipient
    /// @param _deadline The deadline of the tx
    /// @return amountsIn Input amounts for every step of the swap
    function swapAVAXForExactTokens(
        uint256 _amountOut,
        uint256[] memory _pairBinSteps,
        IERC20[] memory _tokenPath,
        address _to,
        uint256 _deadline
    )
        external
        payable
        override
        ensure(_deadline)
        verifyInputs(_pairBinSteps, _tokenPath)
        returns (uint256[] memory amountsIn)
    {
        if (_tokenPath[0] != IERC20(wavax)) revert LBRouter__InvalidTokenPath(address(_tokenPath[0]));

        address[] memory _pairs = _getPairs(_pairBinSteps, _tokenPath);
        amountsIn = _getAmountsIn(_pairBinSteps, _pairs, _tokenPath, _amountOut);

        if (amountsIn[0] > msg.value) revert LBRouter__MaxAmountInExceeded(msg.value, amountsIn[0]);

        _wavaxDepositAndTransfer(_pairs[0], amountsIn[0]);

        uint256 _amountOutReal = _swapTokensForExactTokens(_pairs, _pairBinSteps, _tokenPath, amountsIn, _to);

        if (_amountOutReal < _amountOut) revert LBRouter__InsufficientAmountOut(_amountOut, _amountOutReal);

        if (msg.value > amountsIn[0]) _safeTransferAVAX(_to, amountsIn[0] - msg.value);
    }

    /// @notice Swaps exact tokens for tokens while performing safety checks supporting for fee on transfer tokens
    /// @param _amountIn The amount of token to send
    /// @param _amountOutMin The min amount of token to receive
    /// @param _pairBinSteps The bin step of the pairs (0: V1, other values will use V2)
    /// @param _tokenPath The swap path using the binSteps following `_pairBinSteps`
    /// @param _to The address of the recipient
    /// @param _deadline The deadline of the tx
    /// @return amountOut Output amount of the swap
    function swapExactTokensForTokensSupportingFeeOnTransferTokens(
        uint256 _amountIn,
        uint256 _amountOutMin,
        uint256[] memory _pairBinSteps,
        IERC20[] memory _tokenPath,
        address _to,
        uint256 _deadline
    ) external override ensure(_deadline) verifyInputs(_pairBinSteps, _tokenPath) returns (uint256 amountOut) {
        address[] memory _pairs = _getPairs(_pairBinSteps, _tokenPath);

        IERC20 _targetToken = _tokenPath[_pairs.length];

        uint256 _balanceBefore = _targetToken.balanceOf(_to);

        _tokenPath[0].safeTransferFrom(msg.sender, _pairs[0], _amountIn);

        _swapSupportingFeeOnTransferTokens(_pairs, _pairBinSteps, _tokenPath, _to);

        amountOut = _targetToken.balanceOf(_to) - _balanceBefore;
        if (_amountOutMin > amountOut) revert LBRouter__InsufficientAmountOut(_amountOutMin, amountOut);
    }

    /// @notice Swaps exact tokens for AVAX while performing safety checks supporting for fee on transfer tokens
    /// @param _amountIn The amount of token to send
    /// @param _amountOutMinAVAX The min amount of AVAX to receive
    /// @param _pairBinSteps The bin step of the pairs (0: V1, other values will use V2)
    /// @param _tokenPath The swap path using the binSteps following `_pairBinSteps`
    /// @param _to The address of the recipient
    /// @param _deadline The deadline of the tx
    /// @return amountOut Output amount of the swap
    function swapExactTokensForAVAXSupportingFeeOnTransferTokens(
        uint256 _amountIn,
        uint256 _amountOutMinAVAX,
        uint256[] memory _pairBinSteps,
        IERC20[] memory _tokenPath,
        address payable _to,
        uint256 _deadline
    ) external override ensure(_deadline) verifyInputs(_pairBinSteps, _tokenPath) returns (uint256 amountOut) {
        if (_tokenPath[_pairBinSteps.length] != IERC20(wavax))
            revert LBRouter__InvalidTokenPath(address(_tokenPath[_pairBinSteps.length]));

        address[] memory _pairs = _getPairs(_pairBinSteps, _tokenPath);

        uint256 _balanceBefore = wavax.balanceOf(address(this));

        _tokenPath[0].safeTransferFrom(msg.sender, _pairs[0], _amountIn);

        _swapSupportingFeeOnTransferTokens(_pairs, _pairBinSteps, _tokenPath, address(this));

        amountOut = wavax.balanceOf(address(this)) - _balanceBefore;
        if (_amountOutMinAVAX > amountOut) revert LBRouter__InsufficientAmountOut(_amountOutMinAVAX, amountOut);

        wavax.withdraw(amountOut);
        _safeTransferAVAX(_to, amountOut);
    }

    /// @notice Swaps exact AVAX for tokens while performing safety checks supporting for fee on transfer tokens
    /// @param _amountOutMin The min amount of token to receive
    /// @param _pairBinSteps The bin step of the pairs (0: V1, other values will use V2)
    /// @param _tokenPath The swap path using the binSteps following `_pairBinSteps`
    /// @param _to The address of the recipient
    /// @param _deadline The deadline of the tx
    /// @return amountOut Output amount of the swap
    function swapExactAVAXForTokensSupportingFeeOnTransferTokens(
        uint256 _amountOutMin,
        uint256[] memory _pairBinSteps,
        IERC20[] memory _tokenPath,
        address _to,
        uint256 _deadline
    ) external payable override ensure(_deadline) verifyInputs(_pairBinSteps, _tokenPath) returns (uint256 amountOut) {
        if (_tokenPath[0] != IERC20(wavax)) revert LBRouter__InvalidTokenPath(address(_tokenPath[0]));

        address[] memory _pairs = _getPairs(_pairBinSteps, _tokenPath);

        IERC20 _targetToken = _tokenPath[_pairs.length];

        uint256 _balanceBefore = _targetToken.balanceOf(_to);

        _wavaxDepositAndTransfer(_pairs[0], msg.value);

        _swapSupportingFeeOnTransferTokens(_pairs, _pairBinSteps, _tokenPath, _to);

        amountOut = _targetToken.balanceOf(_to) - _balanceBefore;
        if (_amountOutMin > amountOut) revert LBRouter__InsufficientAmountOut(_amountOutMin, amountOut);
    }

    /// @notice Unstuck tokens that are sent to this contract by mistake
    /// @dev Only callable by the factory owner
    /// @param _token The address of the token
    /// @param _to The address of the user to send back the tokens
    /// @param _amount The amount to send
    function sweep(
        IERC20 _token,
        address _to,
        uint256 _amount
    ) external override onlyFactoryOwner {
        if (address(_token) == address(0)) {
            if (_amount == type(uint256).max) _amount = address(this).balance;
            _safeTransferAVAX(_to, _amount);
        } else {
            if (_amount == type(uint256).max) _amount = _token.balanceOf(address(this));
            _token.safeTransfer(_to, _amount);
        }
    }

    /// @notice Unstuck LBTokens that are sent to this contract by mistake
    /// @dev Only callable by the factory owner
    /// @param _lbToken The address of the LBToken
    /// @param _to The address of the user to send back the tokens
    /// @param _ids The list of token ids
    /// @param _amounts The list of amounts to send
    function sweepLBToken(
        ILBToken _lbToken,
        address _to,
        uint256[] memory _ids,
        uint256[] memory _amounts
    ) external override onlyFactoryOwner {
        _lbToken.safeBatchTransferFrom(address(this), _to, _ids, _amounts);
    }

    /// @notice Helper function to add liquidity
    /// @param _liq The liquidity parameter
    /// @param _LBPair LBPair where liquidity is deposited
    /// @return depositIds Bin ids where the liquidity was actually deposited
    /// @return liquidityMinted Amounts of LBToken minted for each bin
    function _addLiquidity(LiquidityParameters memory _liq, ILBPair _LBPair)
        private
        ensure(_liq.deadline)
        returns (uint256[] memory depositIds, uint256[] memory liquidityMinted)
    {
        unchecked {
            if (_liq.deltaIds.length != _liq.distributionX.length && _liq.deltaIds.length != _liq.distributionY.length)
                revert LBRouter__LengthsMismatch();

            if (_liq.activeIdDesired > type(uint24).max || _liq.idSlippage > type(uint24).max)
                revert LBRouter__IdDesiredOverflows(_liq.activeIdDesired, _liq.idSlippage);

            (, , uint256 _activeId) = _LBPair.getReservesAndId();
            if (
                _liq.activeIdDesired + _liq.idSlippage < _activeId || _activeId + _liq.idSlippage < _liq.activeIdDesired
            ) revert LBRouter__IdSlippageCaught(_liq.activeIdDesired, _liq.idSlippage, _activeId);

            depositIds = new uint256[](_liq.deltaIds.length);
            for (uint256 i; i < depositIds.length; ++i) {
                int256 _id = int256(_activeId) + _liq.deltaIds[i];
                if (_id < 0 || uint256(_id) > type(uint24).max) revert LBRouter__IdOverflows(_id);
                depositIds[i] = uint256(_id);
            }

            uint256 _amountXAdded;
            uint256 _amountYAdded;

            (_amountXAdded, _amountYAdded, liquidityMinted) = _LBPair.mint(
                depositIds,
                _liq.distributionX,
                _liq.distributionY,
                _liq.to
            );

            if (_amountXAdded < _liq.amountXMin || _amountYAdded < _liq.amountYMin)
                revert LBRouter__AmountSlippageCaught(_liq.amountXMin, _amountXAdded, _liq.amountYMin, _amountYAdded);
        }
    }

    /// @notice Helper function to return the amounts in
    /// @param _pairBinSteps The bin step of the pairs (0: V1, other values will use V2)
    /// @param _pairs The list of pairs
    /// @param _tokenPath The swap path
    /// @param _amountOut The amount out
    /// @return amountsIn The list of amounts in
    function _getAmountsIn(
        uint256[] memory _pairBinSteps,
        address[] memory _pairs,
        IERC20[] memory _tokenPath,
        uint256 _amountOut
    ) private view returns (uint256[] memory amountsIn) {
        amountsIn = new uint256[](_tokenPath.length);
        // Avoid doing -1, as `_pairs.length == _pairBinSteps.length-1`
        amountsIn[_pairs.length] = _amountOut;

        for (uint256 i = _pairs.length; i != 0; i--) {
            IERC20 _token = _tokenPath[i - 1];
            uint256 _binStep = _pairBinSteps[i - 1];

            address _pair = _pairs[i - 1];

            if (_binStep == 0) {
                (uint256 _reserveIn, uint256 _reserveOut, ) = IJoePair(_pair).getReserves();
                if (_token > _tokenPath[i]) {
                    (_reserveIn, _reserveOut) = (_reserveOut, _reserveIn);
                }

                uint256 amountOut_ = amountsIn[i];
                // Legacy uniswap way of rounding
                amountsIn[i - 1] = (_reserveIn * amountOut_ * 1_000) / (_reserveOut - amountOut_ * 997) + 1;
            } else {
                (amountsIn[i - 1], ) = getSwapIn(ILBPair(_pair), amountsIn[i], ILBPair(_pair).tokenX() == _token);
            }
        }
    }

    /// @notice Helper function to remove liquidity
    /// @param _LBPair The address of the LBPair
    /// @param _amountXMin The min amount to receive of token X
    /// @param _amountYMin The min amount to receive of token Y
    /// @param _ids The list of ids to burn
    /// @param _amounts The list of amounts to burn of each id in `_ids`
    /// @param _to The address of the recipient
    /// @param amountX The amount of token X sent by the pair
    /// @param amountY The amount of token Y sent by the pair
    function _removeLiquidity(
        ILBPair _LBPair,
        uint256 _amountXMin,
        uint256 _amountYMin,
        uint256[] memory _ids,
        uint256[] memory _amounts,
        address _to
    ) private returns (uint256 amountX, uint256 amountY) {
        ILBToken(address(_LBPair)).safeBatchTransferFrom(msg.sender, address(_LBPair), _ids, _amounts);
        (amountX, amountY) = _LBPair.burn(_ids, _amounts, _to);
        if (amountX < _amountXMin || amountY < _amountYMin)
            revert LBRouter__AmountSlippageCaught(_amountXMin, amountX, _amountYMin, amountY);
    }

    /// @notice Helper function to swap exact tokens for tokens
    /// @param _amountIn The amount of token sent
    /// @param _pairs The list of pairs
    /// @param _pairBinSteps The bin step of the pairs (0: V1, other values will use V2)
    /// @param _tokenPath The swap path using the binSteps following `_pairBinSteps`
    /// @param _to The address of the recipient
    /// @return amountOut The amount of token sent to `_to`
    function _swapExactTokensForTokens(
        uint256 _amountIn,
        address[] memory _pairs,
        uint256[] memory _pairBinSteps,
        IERC20[] memory _tokenPath,
        address _to
    ) private returns (uint256 amountOut) {
        IERC20 _token;
        uint256 _binStep;
        address _recipient;
        address _pair;

        IERC20 _tokenNext = _tokenPath[0];
        amountOut = _amountIn;

        unchecked {
            for (uint256 i; i < _pairs.length; ++i) {
                _pair = _pairs[i];
                _binStep = _pairBinSteps[i];

                _token = _tokenNext;
                _tokenNext = _tokenPath[i + 1];

                _recipient = i + 1 == _pairs.length ? _to : _pairs[i + 1];

                if (_binStep == 0) {
                    (uint256 _reserve0, uint256 _reserve1, ) = IJoePair(_pair).getReserves();

                    if (_token < _tokenNext) {
                        amountOut = (_reserve1 * amountOut * 997) / (_reserve0 * 1_000 + amountOut * 997);
                        IJoePair(_pair).swap(0, amountOut, _recipient, "");
                    } else {
                        amountOut = (_reserve0 * amountOut * 997) / (_reserve1 * 1_000 + amountOut * 997);
                        IJoePair(_pair).swap(amountOut, 0, _recipient, "");
                    }
                } else {
                    bool _swapForY = _tokenNext == ILBPair(_pair).tokenY();

                    (uint256 _amountXOut, uint256 _amountYOut) = ILBPair(_pair).swap(_swapForY, _recipient);

                    if (_swapForY) amountOut = _amountYOut;
                    else amountOut = _amountXOut;
                }
            }
        }
    }

    /// @notice Helper function to swap tokens for exact tokens
    /// @param _pairs The array of pairs
    /// @param _pairBinSteps The versions of each pair (1: DexV1, 2: dexV2)
    /// @param _tokenPath The swap path using the binSteps following `_pairBinSteps`
    /// @param _amountsIn The list of amounts in
    /// @param _to The address of the recipient
    /// @return amountOut The amount of token sent to `_to`
    function _swapTokensForExactTokens(
        address[] memory _pairs,
        uint256[] memory _pairBinSteps,
        IERC20[] memory _tokenPath,
        uint256[] memory _amountsIn,
        address _to
    ) private returns (uint256 amountOut) {
        IERC20 _token;
        uint256 _binStep;
        address _recipient;
        address _pair;

        IERC20 _tokenNext = _tokenPath[0];

        unchecked {
            for (uint256 i; i < _pairs.length; ++i) {
                _pair = _pairs[i];
                _binStep = _pairBinSteps[i];

                _token = _tokenNext;
                _tokenNext = _tokenPath[i + 1];

                _recipient = i + 1 == _pairs.length ? _to : _pairs[i + 1];

                if (_binStep == 0) {
                    amountOut = _amountsIn[i + 1];
                    if (_token < _tokenNext) {
                        IJoePair(_pair).swap(0, amountOut, _recipient, "");
                    } else {
                        IJoePair(_pair).swap(amountOut, 0, _recipient, "");
                    }
                } else {
                    bool _swapForY = _tokenNext == ILBPair(_pair).tokenY();

                    (uint256 _amountXOut, uint256 _amountYOut) = ILBPair(_pair).swap(_swapForY, _recipient);

                    if (_swapForY) amountOut = _amountYOut;
                    else amountOut = _amountXOut;
                }
            }
        }
    }

    /// @notice Helper function to swap exact tokens supporting for fee on transfer tokens
    /// @param _pairs The list of pairs
    /// @param _pairBinSteps The bin step of the pairs (0: V1, other values will use V2)
    /// @param _tokenPath The swap path using the binSteps following `_pairBinSteps`
    /// @param _to The address of the recipient
    function _swapSupportingFeeOnTransferTokens(
        address[] memory _pairs,
        uint256[] memory _pairBinSteps,
        IERC20[] memory _tokenPath,
        address _to
    ) private {
        IERC20 _token;
        uint256 _binStep;
        address _recipient;
        address _pair;

        IERC20 _tokenNext = _tokenPath[0];

        unchecked {
            for (uint256 i; i < _pairs.length; ++i) {
                _pair = _pairs[i];
                _binStep = _pairBinSteps[i];

                _token = _tokenNext;
                _tokenNext = _tokenPath[i + 1];

                _recipient = i + 1 == _pairs.length ? _to : _pairs[i + 1];

                if (_binStep == 0) {
                    (uint256 _reserve0, uint256 _reserve1, ) = IJoePair(_pair).getReserves();
                    if (_token < _tokenNext) {
                        uint256 _balance = _token.balanceOf(_pair);
                        uint256 _amountOut = (_reserve1 * (_balance - _reserve0) * 997) / (_balance * 1_000);

                        IJoePair(_pair).swap(0, _amountOut, _recipient, "");
                    } else {
                        uint256 _balance = _token.balanceOf(_pair);
                        uint256 _amountOut = (_reserve0 * (_balance - _reserve1) * 997) / (_balance * 1_000);

                        IJoePair(_pair).swap(_amountOut, 0, _recipient, "");
                    }
                } else {
                    ILBPair(_pair).swap(_tokenNext == ILBPair(_pair).tokenY(), _recipient);
                }
            }
        }
    }

    /// @notice Helper function to return the address of the LBPair
    /// @dev Revert if the pair is not created yet
    /// @param _tokenX The address of the tokenX
    /// @param _tokenY The address of the tokenY
    /// @param _binStep The bin step of the LBPair
    /// @return The address of the LBPair
    function _getLBPairInformation(
        IERC20 _tokenX,
        IERC20 _tokenY,
        uint256 _binStep
    ) private view returns (ILBPair) {
        ILBPair _LBPair = factory.getLBPairInformation(_tokenX, _tokenY, _binStep).LBPair;
        if (address(_LBPair) == address(0))
            revert LBRouter__PairNotCreated(address(_tokenX), address(_tokenY), _binStep);
        return _LBPair;
    }

    /// @notice Helper function to return the address of the pair (v1 or v2, according to `_binStep`)
    /// @dev Revert if the pair is not created yet
    /// @param _binStep The bin step of the LBPair, 0 means using V1 pair, any other value will use V2
    /// @param _tokenX The address of the tokenX
    /// @param _tokenY The address of the tokenY
    /// @return _pair The address of the pair of binStep `_binStep`
    function _getPair(
        uint256 _binStep,
        IERC20 _tokenX,
        IERC20 _tokenY
    ) private view returns (address _pair) {
        if (_binStep == 0) {
            _pair = oldFactory.getPair(address(_tokenX), address(_tokenY));
            if (_pair == address(0)) revert LBRouter__PairNotCreated(address(_tokenX), address(_tokenY), _binStep);
        } else _pair = address(_getLBPairInformation(_tokenX, _tokenY, _binStep));
    }

    function _getPairs(uint256[] memory _pairBinSteps, IERC20[] memory _tokenPath)
        private
        view
        returns (address[] memory pairs)
    {
        pairs = new address[](_pairBinSteps.length);

        IERC20 _token;
        IERC20 _tokenNext = _tokenPath[0];
        unchecked {
            for (uint256 i; i < pairs.length; ++i) {
                _token = _tokenNext;
                _tokenNext = _tokenPath[i + 1];

                pairs[i] = _getPair(_pairBinSteps[i], _token, _tokenNext);
            }
        }
    }

    /// @notice Helper function to transfer AVAX
    /// @param _to The address of the recipient
    /// @param _amount The AVAX amount to send
    function _safeTransferAVAX(address _to, uint256 _amount) private {
        (bool success, ) = _to.call{value: _amount}("");
        if (!success) revert LBRouter__FailedToSendAVAX(_to, _amount);
    }

    /// @notice Helper function to deposit and transfer wavax
    /// @param _to The address of the recipient
    /// @param _amount The AVAX amount to wrap
    function _wavaxDepositAndTransfer(address _to, uint256 _amount) private {
        wavax.deposit{value: _amount}();
        wavax.safeTransfer(_to, _amount);
    }
}
