use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

pub fn mint_to(ctx: Context<MintToTokens>, amount: u64) -> Result<()> {
        msg!("Minting {} tokens...", amount);
        msg!("Mint: {}", ctx.accounts.mint.key());
        msg!("Destination Token Account: {}", ctx.accounts.destination.key());
        msg!("Mint Authority: {}", ctx.accounts.mint_authority.key());

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::mint_to(cpi_ctx, amount)?;

        msg!("Tokens minted successfully.");
        Ok(())
    }

#[derive(Accounts)]
pub struct MintToTokens<'info> {
    #[account(
        mut, 
        constraint = mint.mint_authority.unwrap() == mint_authority.key()
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut, 
        constraint = destination.mint == mint.key()
    )]
    pub destination: Account<'info, TokenAccount>,

    
    #[account(mut)] 
    pub mint_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}