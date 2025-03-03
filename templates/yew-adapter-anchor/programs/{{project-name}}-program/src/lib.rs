use anchor_lang::prelude::*;

declare_id!("HAZxVpB2LixYtfbNhdm84PNZrt4CShbv7kjK5Y15y5PS");

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
