use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{instruction, Bet};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    /// CHECK: it's safe because it's checked later
    pub house: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        init,
        payer = player,
        space = 8 + Bet::INIT_SPACE,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,

    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    fn create_bet(
        &mut self,
        seed: u128,
        amount: u64,
        roll: u8,
        bumps: &PlaceBetBumps,
    ) -> Result<()> {
        self.bet.set_inner(Bet {
            player: self.player.key(),
            seed,
            slot: Clock::get()?.slot,
            amount,
            roll,
            bump: bumps.bet,
        });

        Ok(())
    }

    fn deposit(&mut self) -> Result<()> {
        let accounts = Transfer {
            from: self.player.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, self.bet.amount)
    }
}

pub fn handler(ctx: Context<PlaceBet>, seed: u128, amount: u64, roll: u8) -> Result<()> {
    ctx.accounts.create_bet(seed, amount, roll, &ctx.bumps)?;
    ctx.accounts.deposit()?;

    Ok(())
}
