use anchor_lang::prelude::*;

declare_id!("Bkv2uU3t8BR8Se3hA6V6ux1PtHnzdobhdc78QcrWLs3b");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
