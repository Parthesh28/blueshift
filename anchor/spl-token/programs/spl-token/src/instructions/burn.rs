use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};

pub fn burn(ctx: Context<BurnTokens>, amount: u64) -> Result<()>{
msg!("Burning {} tokens...", amount);
        msg!("From Token Account: {}", ctx.accounts.token_account.key());
        msg!("Mint: {}", ctx.accounts.mint.key());
        msg!("Authority: {}", ctx.accounts.authority.key());

        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::burn(cpi_ctx, amount)?;

        msg!("Tokens burned successfully.");
        Ok(())
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(
        mut,
        constraint = mint.key() == token_account.mint

    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = token_account.owner == authority.key()
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)] 
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}