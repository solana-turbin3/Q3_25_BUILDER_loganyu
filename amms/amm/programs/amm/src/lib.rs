#![allow(unexpected_cfgs, deprecated)]

use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;
use state::*;

declare_id!("2f7aXiUHdnAhuBuPLA7wHwaDXf5hkGCnZH1wT9YgpeKY");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, ctx.bumps)?;

        Ok(())
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
        max_x: u64,
        max_y: u64,
    ) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)?;

        Ok(())
    }
}
