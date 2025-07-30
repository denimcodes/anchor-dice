use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{instruction, Bet};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct RefundBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    pub house: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,

    pub system_program: Program<'info, System>,
}

impl<'info> RefundBet<'info> {
    fn withdraw(&mut self) -> Result<()> {
        let seeds = &[
            b"vault",
            self.house.to_account_info().key.as_ref(),
            &[self.bet.bump],
        ];
        let signer = &[&seeds[..]];
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.player.to_account_info(),
        };
        let ctx =
            CpiContext::new_with_signer(self.system_program.to_account_info(), accounts, signer);
        transfer(ctx, self.bet.amount)
    }
}

pub fn handler(ctx: Context<RefundBet>) -> Result<()> {
    ctx.accounts.withdraw()
}
