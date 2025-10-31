use anchor_lang::prelude::*;

declare_id!("3vW7afRuhUc7UFLzb428KQLAvTQpRhXHBPw8tnbLi9ne");

#[program]
pub mod message_saver {

    use super::*;

    pub fn save_message(ctx: Context<SaveMessage>, message: String) -> Result<()> {
        let msg_account = &mut ctx.accounts.message_account;
        msg_account.text = message;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SaveMessage<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 64,
        seeds = [b"message", user.key().as_ref()],
        bump
    )]
    pub message_account: Account<'info, MessageAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct MessageAccount {
    #[max_len(64)]
    pub text: String,
}