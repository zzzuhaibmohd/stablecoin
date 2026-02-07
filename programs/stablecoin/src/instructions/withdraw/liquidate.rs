use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::constants::SEED_CONFIG_ACCOUNT;
use crate::error::StablecoinError;
use crate::instructions::utils::{calculate_health_factor, get_lamports_from_usd};
use crate::instructions::withdraw::{burn_tokens, withdraw_collateral};
use crate::{CollateralState, Config};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config.bump,
        has_one = mint_account,
    )]
    pub config: Account<'info, Config>,

    #[account(mut,
        has_one = sol_account,
    )]
    pub collateral_account: Account<'info, CollateralState>,

    #[account(mut)]
    pub sol_account: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = liquidator,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,

    #[account(mut)]
    pub price_update: Account<'info, PriceUpdateV2>,

    pub system_program: Program<'info, System>,
}

pub fn process_liquidate(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
    let health_factor = calculate_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config,
        &ctx.accounts.price_update,
    )?;

    require!(
        health_factor < ctx.accounts.config.min_health_factor,
        StablecoinError::AboveMinimumHealthFactor
    );

    let lamports = get_lamports_from_usd(&amount_to_burn, &ctx.accounts.price_update)?;

    let liquidation_bonus = lamports
        .checked_mul(ctx.accounts.config.liquidation_bonus)
        .ok_or(StablecoinError::ArithmeticError)?
        .checked_div(100)
        .ok_or(StablecoinError::ArithmeticError)?;

    let amount_to_liquidate = lamports
        .checked_add(liquidation_bonus)
        .ok_or(StablecoinError::ArithmeticError)?;

    withdraw_collateral(
        &ctx.accounts.liquidator.key(),
        &ctx.accounts.sol_account,
        &ctx.accounts.liquidator.to_account_info(),
        amount_to_liquidate,
        &ctx.accounts.system_program,
        ctx.accounts.collateral_account.bump_sol_account,
    )?;

    burn_tokens(
        &ctx.accounts.token_program,
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.liquidator,
        amount_to_burn,
    )?;

    let collateral_account = &mut ctx.accounts.collateral_account;

    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports();
    collateral_account.tokens_minted = collateral_account
        .tokens_minted
        .checked_sub(amount_to_burn)
        .ok_or(StablecoinError::ArithmeticError)?;

    calculate_health_factor(
        collateral_account,
        &ctx.accounts.config,
        &ctx.accounts.price_update,
    )?;

    Ok(())
}
