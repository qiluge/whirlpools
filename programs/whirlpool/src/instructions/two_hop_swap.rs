use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

use crate::state::{TickArray, Whirlpool};

use super::swap;

#[derive(Accounts)]
pub struct TwoHopSwap<'info> {
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    pub token_authority: Signer<'info>,

    #[account(mut)]
    pub whirlpool_one: Box<Account<'info, Whirlpool>>,

    #[account(mut)]
    pub whirlpool_two: Box<Account<'info, Whirlpool>>,

    #[account(mut, constraint = token_owner_account_one_a.mint == whirlpool_one.token_mint_a)]
    pub token_owner_account_one_a: Box<Account<'info, TokenAccount>>,
    #[account(mut, address = whirlpool_one.token_vault_a)]
    pub token_vault_one_a: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = token_owner_account_one_b.mint == whirlpool_one.token_mint_b)]
    pub token_owner_account_one_b: Box<Account<'info, TokenAccount>>,
    #[account(mut, address = whirlpool_one.token_vault_b)]
    pub token_vault_one_b: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = token_owner_account_two_a.mint == whirlpool_two.token_mint_a)]
    pub token_owner_account_two_a: Box<Account<'info, TokenAccount>>,
    #[account(mut, address = whirlpool_two.token_vault_a)]
    pub token_vault_two_a: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = token_owner_account_two_b.mint == whirlpool_two.token_mint_b)]
    pub token_owner_account_two_b: Box<Account<'info, TokenAccount>>,
    #[account(mut, address = whirlpool_two.token_vault_b)]
    pub token_vault_two_b: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = tick_array_one_0.load()?.whirlpool == whirlpool_one.key())]
    pub tick_array_one_0: AccountLoader<'info, TickArray>,

    #[account(mut, constraint = tick_array_one_1.load()?.whirlpool == whirlpool_one.key())]
    pub tick_array_one_1: AccountLoader<'info, TickArray>,

    #[account(mut, constraint = tick_array_one_2.load()?.whirlpool == whirlpool_one.key())]
    pub tick_array_one_2: AccountLoader<'info, TickArray>,

    #[account(mut, constraint = tick_array_two_0.load()?.whirlpool == whirlpool_two.key())]
    pub tick_array_two_0: AccountLoader<'info, TickArray>,

    #[account(mut, constraint = tick_array_two_1.load()?.whirlpool == whirlpool_two.key())]
    pub tick_array_two_1: AccountLoader<'info, TickArray>,

    #[account(mut, constraint = tick_array_two_2.load()?.whirlpool == whirlpool_two.key())]
    pub tick_array_two_2: AccountLoader<'info, TickArray>,

    #[account(seeds = [b"oracle", whirlpool_one.key().as_ref()],bump)]
    /// CHECK: Oracle is currently unused and will be enabled on subsequent updates
    pub oracle_one: UncheckedAccount<'info>,

    #[account(seeds = [b"oracle", whirlpool_two.key().as_ref()],bump)]
    /// CHECK: Oracle is currently unused and will be enabled on subsequent updates
    pub oracle_two: UncheckedAccount<'info>,
}
