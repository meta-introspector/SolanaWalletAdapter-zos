use anchor_lang::prelude::*;

declare_id!("3rFzgYuL3EBragfJhc9ELMUQPeLvP4qApJyWpTBMuZ7g");

#[program]
pub mod temp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
