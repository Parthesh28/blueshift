use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer}; 

pub fn transfer(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
    msg!("Transferring {} tokens...", amount);
    msg!("From: {}", ctx.accounts.from.key());
    msg!("To: {}", ctx.accounts.to.key());
    msg!("Authority: {}", ctx.accounts.authority.key());

    let cpi_accounts = Transfer {
        from: ctx.accounts.from.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::transfer(cpi_ctx, amount)?;

    msg!("Tokens transferred successfully.");
    Ok(())
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(
        mut,
        constraint = from.owner == authority.key()
    )]
    pub from: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = to.mint == from.mint
    )]
    pub to: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
