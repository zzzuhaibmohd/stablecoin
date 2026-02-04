use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct CollateralState {
    pub depositor: Pubkey,     // the depositor of the collateral
    pub sol_account: Pubkey,   // account that holds the SOL from depositor
    pub token_account: Pubkey, // account that holds the stablecoin
    pub lamport_balance: u64,  // tracking lamports for checking healthfactor
    pub tokens_minted: u64,    // tracking stablecoin minted
    pub bump: u8,              // bump for the CollateralState account
    pub bump_sol_account: u8,  // bump for the sol_account
    pub is_initialized: bool,  // whether the account is initialized
}

// @note : token_account is an Associated Token Account (ATA), you don’t need to store the bump. ATAs use a deterministic derivation
// @note : sol_account is a custom PDA created by your program, you need to store its bump to sign for it later.

#[account]
#[derive(InitSpace, Debug)]
pub struct Config {
    pub authority: Pubkey,          // the authority of the config
    pub mint_account: Pubkey,       // the mint account of the stablecoin
    pub liquidation_threshold: u64, // the liquidation threshold of the stablecoin - denotes how much extra collateral is needed
    pub liquidation_bonus: u64, // the liquidation bonus of the stablecoin - denotes how much bonus is given to the liquidator
    pub min_health_factor: u64, // the minimum health factor of the stablecoin - denotes the minimum health factor required for the stablecoin to be considered healthy
    pub bump: u8,               // bump for the Config account
    pub bump_mint_account: u8,  // bump for the mint_account
}
