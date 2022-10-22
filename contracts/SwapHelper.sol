// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "./BinHelper.sol";
import "./Constants.sol";
import "./FeeDistributionHelper.sol";
import "./FeeHelper.sol";
import "./Math512Bits.sol";
import "./SafeMath.sol";
import "../interfaces/ILBPair.sol";

/// @title Liquidity Book Swap Helper Library
/// @author Trader Joe
/// @notice Helper contract used for calculating swaps, fees and reserves changes
library SwapHelper {
    using Math512Bits for uint256;
    using FeeHelper for FeeHelper.FeeParameters;
    using SafeMath for uint256;
    using FeeDistributionHelper for FeeHelper.FeesDistribution;

    /// @notice Returns the swap amounts in the current bin
    /// @param bin The bin information
    /// @param fp The fee parameters
    /// @param activeId The active id of the pair
    /// @param swapForY Whether you've swapping token X for token Y (true) or token Y for token X (false)
    /// @param amountIn The amount sent by the user
    /// @return amountInToBin The amount of token that is added to the bin without the fees
    /// @return amountOutOfBin The amount of token that is removed from the bin
    /// @return fees The swap fees
    function getAmounts(
        ILBPair.Bin memory bin,
        FeeHelper.FeeParameters memory fp,
        uint256 activeId,
        bool swapForY,
        uint256 amountIn
    )
        internal
        pure
        returns (
            uint256 amountInToBin,
            uint256 amountOutOfBin,
            FeeHelper.FeesDistribution memory fees
        )
    {
        uint256 _price = BinHelper.getPriceFromId(activeId, fp.binStep);

        uint256 _reserve;
        uint256 _maxAmountInToBin;
        if (swapForY) {
            _reserve = bin.reserveY;
            _maxAmountInToBin = _reserve.shiftDivRoundUp(Constants.SCALE_OFFSET, _price);
        } else {
            _reserve = bin.reserveX;
            _maxAmountInToBin = _price.mulShiftRoundUp(_reserve, Constants.SCALE_OFFSET);
        }

        fp.updateVolatilityAccumulated(activeId);
        fees = fp.getFeeAmountDistribution(fp.getFeeAmount(_maxAmountInToBin));

        if (_maxAmountInToBin + fees.total <= amountIn) {
            amountInToBin = _maxAmountInToBin;
            amountOutOfBin = _reserve;
        } else {
            fees = fp.getFeeAmountDistribution(fp.getFeeAmountFrom(amountIn));
            amountInToBin = amountIn - fees.total;
            amountOutOfBin = swapForY
                ? _price.mulShiftRoundDown(amountInToBin, Constants.SCALE_OFFSET)
                : amountInToBin.shiftDivRoundDown(Constants.SCALE_OFFSET, _price);
            // Safety check in case rounding returns a higher value than expected
            if (amountOutOfBin > _reserve) amountOutOfBin = _reserve;
        }
    }

    /// @notice Update the fees of the pair and accumulated token per share of the bin
    /// @param bin The bin information
    /// @param pairFees The current fees of the pair information
    /// @param fees The fees amounts added to the pairFees
    /// @param swapForY whether the token sent was Y (true) or X (false)
    /// @param totalSupply The total supply of the token id
    function updateFees(
        ILBPair.Bin memory bin,
        FeeHelper.FeesDistribution memory pairFees,
        FeeHelper.FeesDistribution memory fees,
        bool swapForY,
        uint256 totalSupply
    ) internal pure {
        pairFees.total += fees.total;
        // unsafe math is fine because total >= protocol
        unchecked {
            pairFees.protocol += fees.protocol;
        }

        if (swapForY) {
            bin.accTokenXPerShare += fees.getTokenPerShare(totalSupply);
        } else {
            bin.accTokenYPerShare += fees.getTokenPerShare(totalSupply);
        }
    }

    /// @notice Update reserves
    /// @param bin The bin information
    /// @param pair The pair information
    /// @param swapForY whether the token sent was Y (true) or X (false)
    /// @param amountInToBin The amount of token that is added to the bin without fees
    /// @param amountOutOfBin The amount of token that is removed from the bin
    function updateReserves(
        ILBPair.Bin memory bin,
        ILBPair.PairInformation memory pair,
        bool swapForY,
        uint112 amountInToBin,
        uint112 amountOutOfBin
    ) internal pure {
        if (swapForY) {
            bin.reserveX += amountInToBin;

            unchecked {
                bin.reserveY -= amountOutOfBin;
                pair.reserveX += uint136(amountInToBin);
                pair.reserveY -= uint136(amountOutOfBin);
            }
        } else {
            bin.reserveY += amountInToBin;

            unchecked {
                bin.reserveX -= amountOutOfBin;
                pair.reserveX -= uint136(amountOutOfBin);
                pair.reserveY += uint136(amountInToBin);
            }
        }
    }
}
