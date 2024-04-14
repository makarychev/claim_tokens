use anchor_lang::prelude::*;

use crate::contexts::InitializeRule;

pub fn initialize_rule(
  ctx: Context<InitializeRule>,
  max_claim_amount: u64,
  max_claim_count: u8,
  holding_bump: u8,
) -> Result<()> {
  let rule = &mut ctx.accounts.rule;
  rule.token_address = ctx.accounts.holding_mint.key();
  rule.holding_address = ctx.accounts.holding_address.key();
  rule.max_claim_amount = max_claim_amount;
  rule.total_claimed_amount = 0;
  rule.total_claim_count = 0;
  rule.max_claim_count = max_claim_count;
  rule.bump = holding_bump;

  Ok(())
}