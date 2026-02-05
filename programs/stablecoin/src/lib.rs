use anchor_lang::prelude::*;

use state::*;
mod state;

use constants::*;
mod constants;

use instructions::*;
mod instructions;

use error::*;
mod error;

declare_id!("Bkv2uU3t8BR8Se3hA6V6ux1PtHnzdobhdc78QcrWLs3b");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx);
        Ok(())
    }

    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(ctx, min_health_factor);
        Ok(())
    }

    pub fn deposit_collateral_and_mint_tokens(
        ctx: Context<DepositCollateralAndMintTokens>,
        amount_collateral: u64,
        amount_to_mint: u64,
    ) -> Result<()> {
        process_deposit_collateral_and_mint_tokens(ctx, amount_collateral, amount_to_mint);
        Ok(())
    }

    pub fn redeem_collateral_and_burn_tokens(
        ctx: Context<RedeemCollateralAndBurnTokens>,
        amount_collateral: u64,
        amount_to_burn: u64,
    ) -> Result<()> {
        process_redeem_collateral_and_burn_tokens(ctx, amount_collateral, amount_to_burn);
        Ok(())
    }

    pub fn liquidate(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
        process_liquidate(ctx, amount_to_burn);
        Ok(())
    }
}
