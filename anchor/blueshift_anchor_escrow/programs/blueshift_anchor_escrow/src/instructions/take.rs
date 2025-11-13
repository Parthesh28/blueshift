use crate::errors::EscrowError;
use crate::state::Escrow;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

pub fn transfer_to_maker(accounts: &Take) -> Result<()> {
    transfer_checked(
        CpiContext::new(
            accounts.token_program.to_account_info(),
            TransferChecked {
                from: accounts.taker_ata_b.to_account_info(),
                to: accounts.maker_ata_b.to_account_info(),
                mint: accounts.mint_b.to_account_info(),
                authority: accounts.taker.to_account_info(),
            },
        ),
        accounts.escrow.receive,
        accounts.mint_b.decimals,
    )?;

    Ok(())
}

pub fn withdraw_and_close_vault(accounts: &Take) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"escrow",
        accounts.maker.to_account_info().key.as_ref(),
        &accounts.escrow.seed.to_le_bytes()[..],
        &[accounts.escrow.bump],
    ]];

    transfer_checked(
        CpiContext::new_with_signer(
            accounts.token_program.to_account_info(),
            TransferChecked {
                from: accounts.vault.to_account_info(),
                to: accounts.taker_ata_a.to_account_info(),
                mint: accounts.mint_a.to_account_info(),
                authority: accounts.escrow.to_account_info(),
            },
            &signer_seeds,
        ),
        accounts.vault.amount,
        accounts.mint_a.decimals,
    )?;

    close_account(CpiContext::new_with_signer(
        accounts.token_program.to_account_info(),
        CloseAccount {
            account: accounts.vault.to_account_info(),
            authority: accounts.escrow.to_account_info(),
            destination: accounts.maker.to_account_info(),
        },
        &signer_seeds,
    ))?;

    Ok(())
}

pub fn handler(mut ctx: Context<Take>) -> Result<()> {
    transfer_to_maker(&mut ctx.accounts)?;
    withdraw_and_close_vault(&mut ctx.accounts)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(
      mut,
      close = maker,
      seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
      bump = escrow.bump,
      has_one = maker @ EscrowError::InvalidMaker,
      has_one = mint_a @ EscrowError::InvalidMintA,
      has_one = mint_b @ EscrowError::InvalidMintB,
  )]
    pub escrow: Box<Account<'info, Escrow>>,

    pub mint_a: Box<InterfaceAccount<'info, Mint>>,

    pub mint_b: Box<InterfaceAccount<'info, Mint>>,
    #[account(
      mut,
      associated_token::mint = mint_a,
      associated_token::authority = escrow,
      associated_token::token_program = token_program
  )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
      init_if_needed,
      payer = taker,
      associated_token::mint = mint_a,
      associated_token::authority = taker,
      associated_token::token_program = token_program
  )]
    pub taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
      mut,
      associated_token::mint = mint_b,
      associated_token::authority = taker,
      associated_token::token_program = token_program
  )]
    pub taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
      init_if_needed,
      payer = taker,
      associated_token::mint = mint_b,
      associated_token::authority = maker,
      associated_token::token_program = token_program
  )]
    pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}
