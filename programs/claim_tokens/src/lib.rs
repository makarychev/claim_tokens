use anchor_lang::prelude::*;

declare_id!("52zV7g7WwxLu93W9dxrTr1dvZWD9ZAs7tXHoLASaP6p5");

#[program]
pub mod claim_tokens {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
