use anchor_lang::prelude::*;

mod instructions;
mod contexts;

use contexts::*;


declare_id!("52zV7g7WwxLu93W9dxrTr1dvZWD9ZAs7tXHoLASaP6p5");

#[program]
pub mod claim_tokens {
    use super::*;

    pub fn initialize_rule(  
        ctx: Context<InitializeRule>,
        max_claim_amount: u64,
        max_claim_count: u8,
        holding_bump: u8,
    ) -> Result<()> {
        instructions::initialize::initialize_rule(
            ctx,
            max_claim_amount,
            max_claim_count,
            holding_bump,
        )
    }
}
