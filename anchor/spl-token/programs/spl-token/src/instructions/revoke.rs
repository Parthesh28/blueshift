use anchor_lang::prelude::*;
use anchor_spl::token::{self, Revoke, Token, TokenAccount};

pub fn revoke(ctx: Context<RevokeDelegation>) -> Result<()>{
    msg!("Revoking delegation from account: {}", ctx.accounts.token_account.key());
        msg!("Authority (owner): {}", ctx.accounts.authority.key());

        let cpi_accounts = Revoke {
            source: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::revoke(cpi_ctx)?;

        msg!("Delegation revoked successfully.");
        Ok(())
    
}

#[derive(Accounts)]
pub struct RevokeDelegation<'info> {
    #[account(
        mut,
        constraint = token_account.owner == authority.key()
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}