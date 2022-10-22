// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "./LBErrors.sol";
import "./libraries/BinHelper.sol";
import "./libraries/JoeLibrary.sol";
import "./libraries/Math512Bits.sol";
import "./interfaces/IJoeFactory.sol";
import "./interfaces/IJoePair.sol";
import "./interfaces/ILBFactory.sol";
import "./interfaces/ILBRouter.sol";

/// @title Liquidity Book Quoter
/// @author Trader Joe
/// @notice Helper contract to determine best path through multiple markets
contract LBQuoter {
    using Math512Bits for uint256;

    /// @notice Dex V2 router address
    address public immutable routerV2;
    /// @notice Dex V1 factory address
    address public immutable factoryV1;
    /// @notice Dex V2 factory address
    address public immutable factoryV2;

    struct Quote {
        address[] route;
        address[] pairs;
        uint256[] binSteps;
        uint256[] amounts;
        uint256[] virtualAmountsWithoutSlippage;
        uint256[] fees;
    }

    /// @notice Constructor
    /// @param _routerV2 Dex V2 router address
    /// @param _factoryV1 Dex V1 factory address
    /// @param _factoryV2 Dex V2 factory address
    constructor(
        address _routerV2,
        address _factoryV1,
        address _factoryV2
    ) {
        routerV2 = _routerV2;
        factoryV1 = _factoryV1;
        factoryV2 = _factoryV2;
    }

    /// @notice Finds the best path given a list of tokens and the input amount wanted from the swap
    /// @param _route List of the tokens to go through
    /// @param _amountIn Swap amount in
    /// @return quote The Quote structure containing the necessary element to perform the swap
    function findBestPathFromAmountIn(address[] memory _route, uint256 _amountIn)
        public
        view
        returns (Quote memory quote)
    {
        if (_route.length < 2) {
            revert LBQuoter_InvalidLength();
        }

        quote.route = _route;

        uint256 swapLength = _route.length - 1;
        quote.pairs = new address[](swapLength);
        quote.binSteps = new uint256[](swapLength);
        quote.fees = new uint256[](swapLength);
        quote.amounts = new uint256[](_route.length);
        quote.virtualAmountsWithoutSlippage = new uint256[](_route.length);

        quote.amounts[0] = _amountIn;
        quote.virtualAmountsWithoutSlippage[0] = _amountIn;

        for (uint256 i; i < swapLength; i++) {
            // Fetch swap for V1
            quote.pairs[i] = IJoeFactory(factoryV1).getPair(_route[i], _route[i + 1]);

            if (quote.pairs[i] != address(0) && quote.amounts[i] > 0) {
                (uint256 reserveIn, uint256 reserveOut) = _getReserves(quote.pairs[i], _route[i], _route[i + 1]);

                if (reserveIn > 0 && reserveOut > 0) {
                    quote.amounts[i + 1] = JoeLibrary.getAmountOut(quote.amounts[i], reserveIn, reserveOut);
                    quote.virtualAmountsWithoutSlippage[i + 1] = JoeLibrary.quote(
                        (quote.virtualAmountsWithoutSlippage[i] * 997) / 1000,
                        reserveIn,
                        reserveOut
                    );
                    quote.fees[i] = 0.003e18; // 0.3%
                }
            }

            // Fetch swaps for V2
            ILBFactory.LBPairInformation[] memory LBPairsAvailable = ILBFactory(factoryV2).getAllLBPairs(
                IERC20(_route[i]),
                IERC20(_route[i + 1])
            );

            if (LBPairsAvailable.length > 0 && quote.amounts[i] > 0) {
                for (uint256 j; j < LBPairsAvailable.length; j++) {
                    if (!LBPairsAvailable[j].ignoredForRouting) {
                        bool swapForY = address(LBPairsAvailable[j].LBPair.tokenY()) == _route[i + 1];

                        try
                            ILBRouter(routerV2).getSwapOut(LBPairsAvailable[j].LBPair, quote.amounts[i], swapForY)
                        returns (uint256 swapAmountOut, uint256 fees) {
                            if (swapAmountOut > quote.amounts[i + 1]) {
                                quote.amounts[i + 1] = swapAmountOut;
                                quote.pairs[i] = address(LBPairsAvailable[j].LBPair);
                                quote.binSteps[i] = LBPairsAvailable[j].binStep;

                                // Getting current price
                                (, , uint256 activeId) = LBPairsAvailable[j].LBPair.getReservesAndId();
                                quote.virtualAmountsWithoutSlippage[i + 1] = _getV2Quote(
                                    quote.virtualAmountsWithoutSlippage[i] - fees,
                                    activeId,
                                    quote.binSteps[i],
                                    swapForY
                                );

                                quote.fees[i] = (fees * 1e18) / quote.amounts[i]; // fee percentage in amountIn
                            }
                        } catch {}
                    }
                }
            }
        }
    }

    /// @notice Finds the best path given a list of tokens and the output amount wanted from the swap
    /// @param _route List of the tokens to go through
    /// @param _amountOut Swap amount out
    /// @return quote The Quote structure containing the necessary element to perform the swap
    function findBestPathFromAmountOut(address[] memory _route, uint256 _amountOut)
        public
        view
        returns (Quote memory quote)
    {
        if (_route.length < 2) {
            revert LBQuoter_InvalidLength();
        }
        quote.route = _route;

        uint256 swapLength = _route.length - 1;
        quote.pairs = new address[](swapLength);
        quote.binSteps = new uint256[](swapLength);
        quote.fees = new uint256[](swapLength);
        quote.amounts = new uint256[](_route.length);
        quote.virtualAmountsWithoutSlippage = new uint256[](_route.length);

        quote.amounts[swapLength] = _amountOut;
        quote.virtualAmountsWithoutSlippage[swapLength] = _amountOut;

        for (uint256 i = swapLength; i > 0; i--) {
            // Fetch swap for V1
            quote.pairs[i - 1] = IJoeFactory(factoryV1).getPair(_route[i - 1], _route[i]);
            if (quote.pairs[i - 1] != address(0) && quote.amounts[i] > 0) {
                (uint256 reserveIn, uint256 reserveOut) = _getReserves(quote.pairs[i - 1], _route[i - 1], _route[i]);

                if (reserveIn > 0 && reserveOut > quote.amounts[i]) {
                    quote.amounts[i - 1] = JoeLibrary.getAmountIn(quote.amounts[i], reserveIn, reserveOut);
                    quote.virtualAmountsWithoutSlippage[i - 1] =
                        (JoeLibrary.quote(quote.virtualAmountsWithoutSlippage[i], reserveOut, reserveIn) * 1000) /
                        997;

                    quote.fees[i - 1] = 0.003e18; // 0.3%
                }
            }

            // Fetch swaps for V2
            ILBFactory.LBPairInformation[] memory LBPairsAvailable = ILBFactory(factoryV2).getAllLBPairs(
                IERC20(_route[i - 1]),
                IERC20(_route[i])
            );

            if (LBPairsAvailable.length > 0 && quote.amounts[i] > 0) {
                for (uint256 j; j < LBPairsAvailable.length; j++) {
                    if (!LBPairsAvailable[j].ignoredForRouting) {
                        bool swapForY = address(LBPairsAvailable[j].LBPair.tokenY()) == _route[i];
                        try
                            ILBRouter(routerV2).getSwapIn(LBPairsAvailable[j].LBPair, quote.amounts[i], swapForY)
                        returns (uint256 swapAmountIn, uint256 fees) {
                            if (
                                swapAmountIn != 0 && (swapAmountIn < quote.amounts[i - 1] || quote.amounts[i - 1] == 0)
                            ) {
                                quote.amounts[i - 1] = swapAmountIn;
                                quote.pairs[i - 1] = address(LBPairsAvailable[j].LBPair);
                                quote.binSteps[i - 1] = LBPairsAvailable[j].binStep;

                                // Getting current price
                                (, , uint256 activeId) = LBPairsAvailable[j].LBPair.getReservesAndId();
                                quote.virtualAmountsWithoutSlippage[i - 1] =
                                    _getV2Quote(
                                        quote.virtualAmountsWithoutSlippage[i],
                                        activeId,
                                        quote.binSteps[i - 1],
                                        !swapForY
                                    ) +
                                    fees;

                                quote.fees[i - 1] = (fees * 1e18) / quote.amounts[i - 1]; // fee percentage in amountIn
                            }
                        } catch {}
                    }
                }
            }
        }
    }

    /// @dev Forked from JoeLibrary
    /// @dev Doesn't rely on the init code hash of the factory
    /// @param _pair Address of the pair
    /// @param _tokenA Address of token A
    /// @param _tokenB Address of token B
    /// @return reserveA Reserve of token A in the pair
    /// @return reserveB Reserve of token B in the pair
    function _getReserves(
        address _pair,
        address _tokenA,
        address _tokenB
    ) internal view returns (uint256 reserveA, uint256 reserveB) {
        (address token0, ) = JoeLibrary.sortTokens(_tokenA, _tokenB);
        (uint256 reserve0, uint256 reserve1, ) = IJoePair(_pair).getReserves();
        (reserveA, reserveB) = _tokenA == token0 ? (reserve0, reserve1) : (reserve1, reserve0);
    }

    /// @dev Calculates a quote for a V2 pair
    /// @param _amount Amount in to consider
    /// @param _activeId Current active Id of the considred pair
    /// @param _binStep Bin step of the considered pair
    /// @param _swapForY Boolean describing if we are swapping from X to Y or the opposite
    /// @return quote Amount Out if _amount was swapped with no slippage and no fees
    function _getV2Quote(
        uint256 _amount,
        uint256 _activeId,
        uint256 _binStep,
        bool _swapForY
    ) internal pure returns (uint256 quote) {
        if (_swapForY) {
            quote = BinHelper.getPriceFromId(_activeId, _binStep).mulShiftRoundDown(_amount, Constants.SCALE_OFFSET);
        } else {
            quote = _amount.shiftDivRoundDown(Constants.SCALE_OFFSET, BinHelper.getPriceFromId(_activeId, _binStep));
        }
    }
}
