use anchor_lang::prelude::*;

use crate::constants::*;
use crate::Config;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
}

pub fn process_update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
    let config_account = &mut ctx.accounts.config;

    config_account.min_health_factor = min_health_factor;

    Ok(())
}
