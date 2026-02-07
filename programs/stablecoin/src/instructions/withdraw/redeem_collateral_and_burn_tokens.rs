use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::constants::{SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT};
use crate::error::StablecoinError;
use crate::instructions::utils::check_health_factor;
use crate::instructions::withdraw::{burn_tokens, withdraw_collateral};
use crate::{CollateralState, Config};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

#[derive(Accounts)]
pub struct RedeemCollateralAndBurnTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    pub price_update: Account<'info, PriceUpdateV2>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config.bump,
        has_one = mint_account,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump = collateral_state.bump,
        has_one = sol_account,
        has_one = token_account,
    )]
    pub collateral_state: Account<'info, CollateralState>,

    #[account(mut)]
    pub sol_account: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_redeem_collateral_and_burn_tokens(
    ctx: Context<RedeemCollateralAndBurnTokens>,
    amount_collateral: u64,
    amount_to_burn: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_state;

    collateral_account.lamport_balance = ctx
        .accounts
        .sol_account
        .lamports()
        .checked_sub(amount_collateral)
        .ok_or(StablecoinError::ArithmeticError)?;
    collateral_account.tokens_minted = collateral_account
        .tokens_minted
        .checked_sub(amount_to_burn)
        .ok_or(StablecoinError::ArithmeticError)?;

    check_health_factor(
        collateral_account,
        &ctx.accounts.config,
        &ctx.accounts.price_update,
    )?;

    burn_tokens(
        &ctx.accounts.token_program,
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.depositor,
        amount_to_burn,
    )?;

    withdraw_collateral(
        &ctx.accounts.depositor.key(),
        &ctx.accounts.sol_account,
        &ctx.accounts.depositor.to_account_info(),
        amount_collateral,
        &ctx.accounts.system_program,
        ctx.accounts.collateral_state.bump_sol_account,
    )?;

    Ok(())
}
