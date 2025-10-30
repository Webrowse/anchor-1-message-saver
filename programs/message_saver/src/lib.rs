use anchor_lang::prelude::*;

declare_id!("3vW7afRuhUc7UFLzb428KQLAvTQpRhXHBPw8tnbLi9ne");

#[program]
pub mod message_saver {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
