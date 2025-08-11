use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{close_account, CloseAccount},
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::Offer;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_token_account_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> TakeOffer<'info> {
    pub fn send_wanted_tokens_to_maker(&mut self) -> Result<()> {
        let transfer_cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            TransferChecked {
                authority: self.taker.to_account_info(),
                from: self.taker_token_account_b.to_account_info(),
                to: self.maker_token_account_b.to_account_info(),
                mint: self.mint_b.to_account_info(),
            },
        );

        transfer_checked(
            transfer_cpi_ctx,
            self.offer.wanted_amount,
            self.mint_b.decimals,
        )?;

        msg!("Wanted tokens sent to maker");
        Ok(())
    }

    pub fn withdraw_n_close_vault(&self) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"offer",
            self.maker.key.as_ref(),
            &self.offer.id.to_le_bytes(),
            &[self.offer.bump],
        ]];

        let accounts = TransferChecked {
            authority: self.offer.to_account_info(),
            from: self.vault.to_account_info(),
            to: self.taker_token_account_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            authority: self.offer.to_account_info(),
            destination: self.maker.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            &signer_seeds,
        );

        close_account(cpi_context)?;

        msg!("Offer tokens withdrawn & close the vault");
        Ok(())
    }
}
