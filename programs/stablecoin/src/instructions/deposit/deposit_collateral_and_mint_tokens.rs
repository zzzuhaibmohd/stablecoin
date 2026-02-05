use anchor_lang::prelude::*;

use crate::constants::*;
use crate::instructions::deposit::{deposit_collateral, mint_tokens};
use crate::instructions::utils::{calculate_health_factor, check_health_factor};
use crate::{CollateralState, Config};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct DepositCollateralAndMintTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config.bump,
        has_one = mint_account,
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = depositor,
        space = 8 + CollateralState::INIT_SPACE,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump,
    )]
    pub collateral_state: Account<'info, CollateralState>,

    #[account(
        mut,
        seeds = [SEED_SOL_ACCOUNT, depositor.key().as_ref()],
        bump
    )]
    pub sol_account: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint_account,
        associated_token::authority = depositor,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

    pub price_update: Account<'info, PriceUpdateV2>,
}

pub fn process_deposit_collateral_and_mint_tokens(
    ctx: Context<DepositCollateralAndMintTokens>,
    amount_collateral: u64,
    amount_to_mint: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_state;

    collateral_account.lamport_balance = ctx
        .accounts
        .sol_account
        .lamports()
        .checked_add(amount_collateral)
        .unwrap();
    collateral_account.tokens_minted = collateral_account
        .tokens_minted
        .checked_add(amount_to_mint)
        .unwrap();

    if !collateral_account.is_initialized {
        collateral_account.is_initialized = true;
        collateral_account.depositor = ctx.accounts.depositor.key();
        collateral_account.sol_account = ctx.accounts.sol_account.key();
        collateral_account.token_account = ctx.accounts.token_account.key();
        collateral_account.bump = ctx.bumps.collateral_state;
        collateral_account.bump_sol_account = ctx.bumps.sol_account;
    }

    check_health_factor(
        collateral_account,
        &ctx.accounts.config,
        &ctx.accounts.price_update,
    )?;

    deposit_collateral(
        &ctx.accounts.depositor,
        &ctx.accounts.sol_account,
        amount_collateral,
        &ctx.accounts.system_program,
    )?;
    mint_tokens(
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.token_program,
        amount_to_mint,
        ctx.accounts.config.bump_mint_account,
    )?;

    Ok(())
}
