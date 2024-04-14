use anchor_lang::prelude::*;
use std::mem;

use crate::contexts::common::*;
use anchor_spl::token::{Mint, TokenAccount};

pub const HOLDING_PREFIX: &str = "holding";


#[account]
#[derive(Default)]
pub struct Rule {
  pub token_address: Pubkey,
  pub holding_address: Pubkey,
  pub max_claim_amount: u64,
  pub total_claimed_amount: u64,
  pub total_claim_count: u64,
  pub max_claim_count: u8,
  pub bump: u8,
}

impl Rule {
  const TOKEN_ADDRESS_LEN: usize = mem::size_of::<Pubkey>();
  const HOLDING_ADDRESS_LEN: usize = mem::size_of::<Pubkey>();
  const MAX_CLAIM_AMOUNT_LEN: usize = mem::size_of::<u64>();
  const TOTAL_CLAIMED_AMOUNT_LEN: usize = mem::size_of::<u64>();
  const TOTAL_CLAIM_COUNT_LEN: usize = mem::size_of::<u64>();
  const MAX_CLAIM_COUNT_LEN: usize = mem::size_of::<u8>();
  const BUMP_LEN: usize = mem::size_of::<u8>();

  pub fn size() -> usize {
    DISCRIMINATOR_LEN
    + Self::TOKEN_ADDRESS_LEN
    + Self::HOLDING_ADDRESS_LEN
    + Self::MAX_CLAIM_AMOUNT_LEN
    + Self::TOTAL_CLAIMED_AMOUNT_LEN
    + Self::TOTAL_CLAIM_COUNT_LEN
    + Self::MAX_CLAIM_COUNT_LEN
    + Self::BUMP_LEN
  }
}


#[derive(Accounts)]
pub struct InitializeRule<'info> {
  #[account(init, payer = payer, space = Rule::size())]
  pub rule: Account<'info, Rule>,

  #[account(
    constraint = holding_address.owner == holding_owner.key(),
    constraint = holding_address.mint == holding_mint.key(),
  )]
  pub holding_address: Account<'info, TokenAccount>,
  #[account(
    seeds = [HOLDING_PREFIX.as_bytes(), holding_mint.key().as_ref()],
    bump,
  )]
  /// CHECK: pda of ["holding", holding_mint]
  pub holding_owner: AccountInfo<'info>,
  pub holding_mint: Account<'info, Mint>,
  #[account(mut)]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
}
