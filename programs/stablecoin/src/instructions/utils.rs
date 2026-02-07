use anchor_lang::prelude::*;

use crate::error::StablecoinError;
use crate::{
    constants::{MAXIMUM_AGE, PRICE_FEED_ADJUSTMENT_FACTOR, PRICE_FEED_ID},
    CollateralState, Config,
};

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

pub fn check_health_factor(
    collateral: &CollateralState,
    config: &Config,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<()> {
    let health_factor = calculate_health_factor(collateral, config, price_feed)?;
    require!(
        health_factor >= config.min_health_factor,
        StablecoinError::HealthFactorTooLow
    );
    Ok(())
}

pub fn calculate_health_factor(
    collateral: &CollateralState,
    config: &Config,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    let collateral_value_in_usd = get_usd_value(&collateral.lamport_balance, price_feed)?;

    let collateral_adjusted_for_liquidation_threshold = (collateral_value_in_usd as u128)
        .checked_mul(config.liquidation_threshold as u128)
        .ok_or(StablecoinError::ArithmeticError)?
        .checked_div(100)
        .ok_or(StablecoinError::ArithmeticError)?;

    if collateral.tokens_minted == 0 {
        msg!("Health factor is infinite because no tokens minted");
        return Ok(u64::MAX);
    }

    let health_factor = collateral_adjusted_for_liquidation_threshold
        .checked_div(collateral.tokens_minted as u128)
        .ok_or(StablecoinError::ArithmeticError)?;
    Ok(health_factor as u64)
}

pub fn get_usd_value(amount_in_lamports: &u64, price_feed: &Account<PriceUpdateV2>) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(PRICE_FEED_ID)?;

    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;

    require!(price.price > 0, StablecoinError::InvalidPrice);

    let price_in_usd = price.price as u128 * PRICE_FEED_ADJUSTMENT_FACTOR;

    let amount_in_usd = (*amount_in_lamports as u128)
        .checked_mul(price_in_usd)
        .ok_or(StablecoinError::ArithmeticError)?
        .checked_div(LAMPORTS_PER_SOL as u128)
        .ok_or(StablecoinError::ArithmeticError)?;

    Ok(amount_in_usd as u64)
}

pub fn get_lamports_from_usd(
    amount_in_usd: &u64,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(PRICE_FEED_ID)?;

    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;

    require!(price.price > 0, StablecoinError::InvalidPrice);

    let price_in_usd = price.price as u128 * PRICE_FEED_ADJUSTMENT_FACTOR;

    let amount_in_lamports = (*amount_in_usd as u128)
        .checked_mul(LAMPORTS_PER_SOL as u128)
        .ok_or(StablecoinError::ArithmeticError)?
        .checked_div(price_in_usd)
        .ok_or(StablecoinError::ArithmeticError)?;

    Ok(amount_in_lamports as u64)
}
