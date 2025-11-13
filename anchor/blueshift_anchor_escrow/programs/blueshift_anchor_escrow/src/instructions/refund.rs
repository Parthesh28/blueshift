use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::{errors::EscrowError, state::Escrow};

pub fn transfer_from_vault(accounts: &Refund) -> Result<()> {
    transfer_checked(
        CpiContext::new_with_signer(
            accounts.token_program.to_account_info(),
            TransferChecked {
                from: accounts.vault.to_account_info(),
                to: accounts.maker_ata_a.to_account_info(),
                authority: accounts.escrow.to_account_info(),
                mint: accounts.mint_a.to_account_info(),
            },
            &[&[
                b"escrow",
                accounts.maker.key().as_ref(),
                accounts.escrow.seed.to_le_bytes().as_ref(),
                &[accounts.escrow.bump],
            ]],
        ),
        accounts.vault.amount,
        accounts.mint_a.decimals,
    )?;

    close_account(CpiContext::new_with_signer(
        accounts.token_program.to_account_info(),
        CloseAccount {
            account: accounts.vault.to_account_info(),
            destination: accounts.maker.to_account_info(),
            authority: accounts.escrow.to_account_info(),
        },
        &[&[
            b"escrow",
            accounts.maker.key().as_ref(),
            accounts.escrow.seed.to_le_bytes().as_ref(),
            &[accounts.escrow.bump],
        ]],
    ))?;

    Ok(())
}

pub fn handler(mut ctx: Context<Refund>) -> Result<()> {
    transfer_from_vault(&mut ctx.accounts)?;
    Ok(())
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker @ EscrowError::InvalidMaker,
        has_one = mint_a @ EscrowError::InvalidMintA,
    )]
    pub escrow: Account<'info, Escrow>,
 
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

