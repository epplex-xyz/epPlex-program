use anchor_lang::prelude::*;

mod state;
mod errors;
mod ephemeral_context;

use ephemeral_context::*;
declare_id!("AESTfYJoYvW7Wp2yTkbRJU8GZxKHeV8wPDzU3cFHULuL");

#[program]
pub mod epplex_program {
    use super::*;

    pub fn create_rule(
        ctx: Context<ManageRule>,
        seed: u64,
        rule_creator: Pubkey,
        renewal_price: u64,
        treasury: Pubkey,
    ) -> Result<()> {
        ctx.accounts.create_rule(seed, rule_creator, renewal_price, treasury)
    }

    pub fn modify_rule(
        ctx: Context<ManageRule>,
        seed: u64,
        rule_creator: Pubkey,
        renewal_price: u64,
        treasury: Pubkey,
    ) -> Result<()> {
        ctx.accounts.modify_rule(seed, rule_creator, renewal_price, treasury)
    }

    pub fn create_membership(
        ctx: Context<CreateMembership>,
        time: i64,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        ctx.accounts.create(time, name, symbol, uri, ctx.bumps)
    }

    pub fn add_time(
        ctx: Context<ManageTime>,
        time: u64,
    ) -> Result<()> {
        ctx.accounts.add(time)
    }

    pub fn remove_time(
        ctx: Context<ManageTime>,
        time: u64,
    ) -> Result<()> {
        ctx.accounts.remove(time)
    }

    pub fn burn_membership(
        ctx: Context<BurnMembership>,
    ) -> Result<()> {
        ctx.accounts.burn(ctx.bumps)
    }

}
