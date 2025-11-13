use crate::errors::EscrowError;
use crate::state::Escrow;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

pub fn populate_escrow(accounts: &mut Make, seed: u64, amount: u64, bump: u8) -> Result<()> {
    let escrow = &mut accounts.escrow;

    escrow.set_inner(Escrow {
        seed,
        maker: accounts.maker.key(),
        mint_a: accounts.mint_a.key(),
        mint_b: accounts.mint_b.key(),
        receive: amount,
        bump,
    });

    Ok(())
}

// workaround with using accounts instead of context
pub fn deposit_tokens(accounts: &mut Make, amount: u64) -> Result<()> {
    transfer_checked(
        CpiContext::new(
            accounts.token_program.to_account_info(),
            TransferChecked {
                from: accounts.maker_ata_a.to_account_info(),
                mint: accounts.mint_a.to_account_info(),
                to: accounts.vault.to_account_info(),
                authority: accounts.maker.to_account_info(),
            },
        ),
        amount,
        accounts.mint_a.decimals,
    )?;

    Ok(())
}

//make ctx mutable to be able to borrow a referece to ctx.accounts 
pub fn handler(mut ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
    require_gt!(receive, 0, EscrowError::InvalidAmount);
    require_gt!(amount, 0, EscrowError::InvalidAmount);

    let bump = ctx.bumps.escrow;

    // initialize escrow
    populate_escrow(&mut ctx.accounts, seed, receive, bump)?;

    // deposit tokens
    deposit_tokens(&mut ctx.accounts, amount)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        init,
        payer = maker,
        space = Escrow::INIT_SPACE + Escrow::DISCRIMINATOR.len(),
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
