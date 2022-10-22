// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "./interfaces/ILBPair.sol";

/** LBRouter errors */

error LBRouter__SenderIsNotWAVAX();
error LBRouter__PairNotCreated(address tokenX, address tokenY, uint256 binStep);
error LBRouter__WrongAmounts(uint256 amount, uint256 reserve);
error LBRouter__SwapOverflows(uint256 id);
error LBRouter__BrokenSwapSafetyCheck();
error LBRouter__NotFactoryOwner();
error LBRouter__TooMuchTokensIn(uint256 excess);
error LBRouter__BinReserveOverflows(uint256 id);
error LBRouter__IdOverflows(int256 id);
error LBRouter__LengthsMismatch();
error LBRouter__WrongTokenOrder();
error LBRouter__IdSlippageCaught(uint256 activeIdDesired, uint256 idSlippage, uint256 activeId);
error LBRouter__AmountSlippageCaught(uint256 amountXMin, uint256 amountX, uint256 amountYMin, uint256 amountY);
error LBRouter__IdDesiredOverflows(uint256 idDesired, uint256 idSlippage);
error LBRouter__FailedToSendAVAX(address recipient, uint256 amount);
error LBRouter__DeadlineExceeded(uint256 deadline, uint256 currentTimestamp);
error LBRouter__AmountSlippageBPTooBig(uint256 amountSlippage);
error LBRouter__InsufficientAmountOut(uint256 amountOutMin, uint256 amountOut);
error LBRouter__MaxAmountInExceeded(uint256 amountInMax, uint256 amountIn);
error LBRouter__InvalidTokenPath(address wrongToken);
error LBRouter__InvalidVersion(uint256 version);
error LBRouter__WrongAvaxLiquidityParameters(
    address tokenX,
    address tokenY,
    uint256 amountX,
    uint256 amountY,
    uint256 msgValue
);

/** LBToken errors */

error LBToken__SpenderNotApproved(address owner, address spender);
error LBToken__TransferFromOrToAddress0();
error LBToken__MintToAddress0();
error LBToken__BurnFromAddress0();
error LBToken__BurnExceedsBalance(address from, uint256 id, uint256 amount);
error LBToken__LengthMismatch(uint256 accountsLength, uint256 idsLength);
error LBToken__SelfApproval(address owner);
error LBToken__TransferExceedsBalance(address from, uint256 id, uint256 amount);

/** LBFactory errors */

error LBFactory__IdenticalAddresses(IERC20 token);
error LBFactory__QuoteAssetNotWhitelisted(IERC20 quoteAsset);
error LBFactory__QuoteAssetAlreadyWhitelisted(IERC20 quoteAsset);
error LBFactory__AddressZero();
error LBFactory__LBPairAlreadyExists(IERC20 tokenX, IERC20 tokenY, uint256 _binStep);
error LBFactory__LBPairNotCreated(IERC20 tokenX, IERC20 tokenY, uint256 binStep);
error LBFactory__DecreasingPeriods(uint16 filterPeriod, uint16 decayPeriod);
error LBFactory__BaseFactorOverflows(uint16 baseFactor, uint256 max);
error LBFactory__ReductionFactorOverflows(uint16 reductionFactor, uint256 max);
error LBFactory__VariableFeeControlOverflows(uint16 variableFeeControl, uint256 max);
error LBFactory__BaseFeesBelowMin(uint256 baseFees, uint256 minBaseFees);
error LBFactory__FeesAboveMax(uint256 fees, uint256 maxFees);
error LBFactory__BinStepRequirementsBreached(uint256 lowerBound, uint16 binStep, uint256 higherBound);
error LBFactory__ProtocolShareOverflows(uint16 protocolShare, uint256 max);
error LBFactory__FunctionIsLockedForUsers(address user);
error LBFactory__FactoryLockIsAlreadyInTheSameState();
error LBFactory__LBPairIgnoredIsAlreadyInTheSameState();
error LBFactory__BinStepHasNoPreset(uint256 binStep);
error LBFactory__SameFeeRecipient(address feeRecipient);
error LBFactory__SameFlashLoanFee(uint256 flashLoanFee);
error LBFactory__LBPairSafetyCheckFailed(address LBPairImplementation);
error LBFactory__SameImplementation(address LBPairImplementation);
error LBFactory__ImplementationNotSet();

/** LBPair errors */

error LBPair__InsufficientAmounts();
error LBPair__AddressZero();
error LBPair__BrokenSwapSafetyCheck();
error LBPair__CompositionFactorFlawed(uint256 id);
error LBPair__InsufficientLiquidityMinted(uint256 id);
error LBPair__InsufficientLiquidityBurned(uint256 id);
error LBPair__WrongLengths();
error LBPair__OnlyStrictlyIncreasingId();
error LBPair__OnlyFactory();
error LBPair__DistributionsOverflow();
error LBPair__OnlyFeeRecipient(address feeRecipient, address sender);
error LBPair__OracleNotEnoughSample();
error LBPair__FlashLoanCallbackFailed();
error LBPair__AlreadyInitialized();
error LBPair__NewSizeTooSmall(uint256 newSize, uint256 oracleSize);

/** BinHelper errors */

error BinHelper__BinStepOverflows(uint256 bp);
error BinHelper__IdOverflows(int256 id);
error BinHelper__IntOverflows(uint256 id);

/** FeeDistributionHelper errors */

error FeeDistributionHelper__FlashLoanUnderflow(uint256 expectedBalance, uint256 balance);

/** Math128x128 errors */

error Math128x128__PowerUnderflow(uint256 x, int256 y);
error Math128x128__LogUnderflow();

/** Math512Bits errors */

error Math512Bits__MulDivOverflow(uint256 prod1, uint256 denominator);
error Math512Bits__ShiftDivOverflow(uint256 prod1, uint256 denominator);
error Math512Bits__MulShiftOverflow(uint256 prod1, uint256 offset);
error Math512Bits__OffsetOverflows(uint256 offset);

/** Oracle errors */

error Oracle__AlreadyInitialized(uint256 _index);
error Oracle__LookUpTimestampTooOld(uint256 _minTimestamp, uint256 _lookUpTimestamp);
error Oracle__NotInitialized();

/** PendingOwnable errors */

error PendingOwnable__NotOwner();
error PendingOwnable__NotPendingOwner();
error PendingOwnable__PendingOwnerAlreadySet();
error PendingOwnable__NoPendingOwner();
error PendingOwnable__AddressZero();

/** ReentrancyGuardUpgradeable errors */

error ReentrancyGuardUpgradeable__ReentrantCall();
error ReentrancyGuardUpgradeable__AlreadyInitialized();

/** SafeCast errors */

error SafeCast__Exceeds256Bits(uint256 x);
error SafeCast__Exceeds248Bits(uint256 x);
error SafeCast__Exceeds240Bits(uint256 x);
error SafeCast__Exceeds232Bits(uint256 x);
error SafeCast__Exceeds224Bits(uint256 x);
error SafeCast__Exceeds216Bits(uint256 x);
error SafeCast__Exceeds208Bits(uint256 x);
error SafeCast__Exceeds200Bits(uint256 x);
error SafeCast__Exceeds192Bits(uint256 x);
error SafeCast__Exceeds184Bits(uint256 x);
error SafeCast__Exceeds176Bits(uint256 x);
error SafeCast__Exceeds168Bits(uint256 x);
error SafeCast__Exceeds160Bits(uint256 x);
error SafeCast__Exceeds152Bits(uint256 x);
error SafeCast__Exceeds144Bits(uint256 x);
error SafeCast__Exceeds136Bits(uint256 x);
error SafeCast__Exceeds128Bits(uint256 x);
error SafeCast__Exceeds120Bits(uint256 x);
error SafeCast__Exceeds112Bits(uint256 x);
error SafeCast__Exceeds104Bits(uint256 x);
error SafeCast__Exceeds96Bits(uint256 x);
error SafeCast__Exceeds88Bits(uint256 x);
error SafeCast__Exceeds80Bits(uint256 x);
error SafeCast__Exceeds72Bits(uint256 x);
error SafeCast__Exceeds64Bits(uint256 x);
error SafeCast__Exceeds56Bits(uint256 x);
error SafeCast__Exceeds48Bits(uint256 x);
error SafeCast__Exceeds40Bits(uint256 x);
error SafeCast__Exceeds32Bits(uint256 x);
error SafeCast__Exceeds24Bits(uint256 x);
error SafeCast__Exceeds16Bits(uint256 x);
error SafeCast__Exceeds8Bits(uint256 x);

/** TreeMath errors */

error TreeMath__ErrorDepthSearch();

/** JoeLibrary errors */

error JoeLibrary__IdenticalAddresses();
error JoeLibrary__AddressZero();
error JoeLibrary__InsufficientAmount();
error JoeLibrary__InsufficientLiquidity();

/** TokenHelper errors */

error TokenHelper__TransferFailed(IERC20 token, address recipient, uint256 amount);

/** LBQuoter errors */

error LBQuoter_InvalidLength();
