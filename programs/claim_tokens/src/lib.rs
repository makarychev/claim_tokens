use anchor_lang::prelude::*;

mod instructions;
mod contexts;

use contexts::*;


declare_id!("52zV7g7WwxLu93W9dxrTr1dvZWD9ZAs7tXHoLASaP6p5");

#[program]
pub mod claim_tokens {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::initialize(ctx)
    }
}
