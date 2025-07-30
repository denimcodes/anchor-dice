use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use anchor_lang::{
    prelude::*,
    solana_program::{keccak::hash, sysvar::instructions::load_instruction_at_checked},
    system_program::{transfer, Transfer},
};

use crate::{error::DiceError, Bet, HOUSE_EDGE};

#[derive(Accounts)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,
    #[account(mut)]
    pub player: UncheckedAccount<'info>,
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
    pub instruction_sysvar: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {
    fn verify_ed25519_signature(&mut self, _sig: &[u8]) -> Result<()> {
        let ix = load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info())?;

        let signatures = Ed25519InstructionSignatures::unpack(&ix.data).unwrap().0;

        let _signature = &signatures[0];

        Ok(())
    }

    fn resolve_bet(&mut self, bumps: ResolveBetBumps, sig: &[u8]) -> Result<()> {
        let hash = hash(sig).to_bytes();
        let mut hash_16: [u8; 16] = [0; 16];
        hash_16.copy_from_slice(&hash[..16]);
        let lower = u128::from_be_bytes(hash_16);
        hash_16.copy_from_slice(&hash[..16]);
        let upper = u128::from_le_bytes(hash_16);

        let roll = lower.wrapping_add(upper).wrapping_rem(100) as u8 + 1;

        if self.bet.roll > roll {
            let payout = (self.bet.amount as u128)
                .checked_mul(10000 - HOUSE_EDGE as u128)
                .ok_or(DiceError::Overflow)?
                .checked_div(self.bet.roll as u128 - 1)
                .ok_or(DiceError::Overflow)?
                .checked_div(100)
                .ok_or(DiceError::Overflow)?;

            let accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            };
            let seeds = &[
                b"house",
                self.house.to_account_info().key.as_ref(),
                &[bumps.vault],
            ];
            let signer = &[&seeds[..]];
            let ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                accounts,
                signer,
            );
            transfer(ctx, payout as u64)?;
        }

        Ok(())
    }
}

pub fn handler(ctx: Context<ResolveBet>, sig: &[u8]) -> Result<()> {
    ctx.accounts.verify_ed25519_signature(sig)?;
    ctx.accounts.resolve_bet(ctx.bumps, sig)?;

    Ok(())
}
