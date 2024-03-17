//! A concentrated liquidity AMM contract powered by Orca.
use anchor_lang::prelude::*;

declare_id!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

#[doc(hidden)]
pub mod constants;
#[doc(hidden)]
pub mod errors;
#[doc(hidden)]
pub mod instructions;
#[doc(hidden)]
pub mod manager;
#[doc(hidden)]
pub mod math;
pub mod state;
#[doc(hidden)]
pub mod util;

use instructions::*;

#[program]
pub mod whirlpool {
    use super::*;

    /// Perform a swap in this Whirlpool
    ///
    /// ### Authority
    /// - "token_authority" - The authority to withdraw tokens from the input token account.
    ///
    /// ### Parameters
    /// - `amount` - The amount of input or output token to swap from (depending on amount_specified_is_input).
    /// - `other_amount_threshold` - The maximum/minimum of input/output token to swap into (depending on amount_specified_is_input).
    /// - `sqrt_price_limit` - The maximum/minimum price the swap will swap to.
    /// - `amount_specified_is_input` - Specifies the token the parameter `amount`represents. If true, the amount represents the input token of the swap.
    /// - `a_to_b` - The direction of the swap. True if swapping from A to B. False if swapping from B to A.
    ///
    /// #### Special Errors
    /// - `ZeroTradableAmount` - User provided parameter `amount` is 0.
    /// - `InvalidSqrtPriceLimitDirection` - User provided parameter `sqrt_price_limit` does not match the direction of the trade.
    /// - `SqrtPriceOutOfBounds` - User provided parameter `sqrt_price_limit` is over Whirlppool's max/min bounds for sqrt-price.
    /// - `InvalidTickArraySequence` - User provided tick-arrays are not in sequential order required to proceed in this trade direction.
    /// - `TickArraySequenceInvalidIndex` - The swap loop attempted to access an invalid array index during the query of the next initialized tick.
    /// - `TickArrayIndexOutofBounds` - The swap loop attempted to access an invalid array index during tick crossing.
    /// - `LiquidityOverflow` - Liquidity value overflowed 128bits during tick crossing.
    /// - `InvalidTickSpacing` - The swap pool was initialized with tick-spacing of 0.
    pub fn swap(
        ctx: Context<Swap>,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
        amount_specified_is_input: bool,
        a_to_b: bool,
    ) -> Result<()> {
        Ok(())
    }

    /// Perform a two-hop swap in this Whirlpool
    ///
    /// ### Authority
    /// - "token_authority" - The authority to withdraw tokens from the input token account.
    ///
    /// ### Parameters
    /// - `amount` - The amount of input or output token to swap from (depending on amount_specified_is_input).
    /// - `other_amount_threshold` - The maximum/minimum of input/output token to swap into (depending on amount_specified_is_input).
    /// - `amount_specified_is_input` - Specifies the token the parameter `amount`represents. If true, the amount represents the input token of the swap.
    /// - `a_to_b_one` - The direction of the swap of hop one. True if swapping from A to B. False if swapping from B to A.
    /// - `a_to_b_two` - The direction of the swap of hop two. True if swapping from A to B. False if swapping from B to A.
    /// - `sqrt_price_limit_one` - The maximum/minimum price the swap will swap to in the first hop.
    /// - `sqrt_price_limit_two` - The maximum/minimum price the swap will swap to in the second hop.
    ///
    /// #### Special Errors
    /// - `ZeroTradableAmount` - User provided parameter `amount` is 0.
    /// - `InvalidSqrtPriceLimitDirection` - User provided parameter `sqrt_price_limit` does not match the direction of the trade.
    /// - `SqrtPriceOutOfBounds` - User provided parameter `sqrt_price_limit` is over Whirlppool's max/min bounds for sqrt-price.
    /// - `InvalidTickArraySequence` - User provided tick-arrays are not in sequential order required to proceed in this trade direction.
    /// - `TickArraySequenceInvalidIndex` - The swap loop attempted to access an invalid array index during the query of the next initialized tick.
    /// - `TickArrayIndexOutofBounds` - The swap loop attempted to access an invalid array index during tick crossing.
    /// - `LiquidityOverflow` - Liquidity value overflowed 128bits during tick crossing.
    /// - `InvalidTickSpacing` - The swap pool was initialized with tick-spacing of 0.
    /// - `InvalidIntermediaryMint` - Error if the intermediary mint between hop one and two do not equal.
    /// - `DuplicateTwoHopPool` - Error if whirlpool one & two are the same pool.
    pub fn two_hop_swap(
        ctx: Context<TwoHopSwap>,
        amount: u64,
        other_amount_threshold: u64,
        amount_specified_is_input: bool,
        a_to_b_one: bool,
        a_to_b_two: bool,
        sqrt_price_limit_one: u128,
        sqrt_price_limit_two: u128,
    ) -> Result<()> {
        Ok(())
    }
}
