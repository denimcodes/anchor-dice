#![allow(unexpected_cfgs, deprecated, ambiguous_glob_reexports)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DBLwxifeUsw1cJNRk2HaMb9kLhCQui7mWeR1M7xxe33w");

#[program]
pub mod anchor_dice {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        instructions::initialize::handler(ctx, amount)
    }
}
