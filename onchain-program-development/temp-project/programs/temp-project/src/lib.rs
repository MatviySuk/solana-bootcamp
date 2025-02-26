use anchor_lang::prelude::*;

declare_id!("8bv6ToM95iKH6HTD7XoehGxyEhLjaLGoyfDM3NNUTNPc");

#[program]
pub mod temp_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
