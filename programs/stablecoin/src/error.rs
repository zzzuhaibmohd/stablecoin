use anchor_lang::prelude::*;

#[error_code]
pub enum StablecoinError {
    #[msg("Invalid price from Pyth oracle")]
    InvalidPrice,
    #[msg("Health factor is too low")]
    HealthFactorTooLow,
    #[msg("Health factor is too high")]
    AboveMinimumHealthFactor,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Arithmetic overflow or underflow")]
    ArithmeticError,
}
