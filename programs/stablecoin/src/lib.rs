use anchor_lang::prelude::*;

use state::*;
mod state;

use constants::*;
mod constants;

use instructions::*;
mod instructions;

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
}
