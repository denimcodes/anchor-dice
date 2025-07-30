use anchor_lang::prelude::*;

#[error_code]
pub enum DiceError {
    #[msg("Math overflow")]
    Overflow,
}
