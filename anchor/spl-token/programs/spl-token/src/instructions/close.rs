use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Token, TokenAccount};

pub fn close(ctx: Context<CloseTokenAccount>)-> Result<()>{
msg!("Closing token account: {}", ctx.accounts.token_account.key());
        msg!("Sending rent to: {}", ctx.accounts.destination.key());

        let cpi_accounts = CloseAccount {
            account: ctx.accounts.token_account.to_account_info(),
            destination: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::close_account(cpi_ctx)?;

        msg!("Account closed and rent reclaimed.");
        Ok(())
}

#[derive(Accounts)]

pub struct CloseTokenAccount<'info> {
    #[account(
        mut, 
        constraint = token_account.amount == 0,
        constraint = token_account.owner == authority.key()
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub destination: AccountInfo<'info>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}