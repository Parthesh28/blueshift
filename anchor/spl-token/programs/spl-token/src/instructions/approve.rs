use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Token, TokenAccount};

pub fn approve(ctx: Context<ApproveTokens>, amount: u64) -> Result<()>{
    msg!("Approving delegate: {}", ctx.accounts.delegate.key());
        msg!("From token account: {}", ctx.accounts.token_account.key());
        msg!("For amount: {}", amount);

        let cpi_accounts = Approve {
            to: ctx.accounts.token_account.to_account_info(),
            delegate: ctx.accounts.delegate.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::approve(cpi_ctx, amount)?;

        msg!("Delegation successful.");
        Ok(())
}

#[derive(Accounts)]
pub struct ApproveTokens<'info> {
    #[account(
        mut, 
      constraint = token_account.owner == authority.key()
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub delegate: AccountInfo<'info>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}