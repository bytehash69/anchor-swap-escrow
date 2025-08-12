use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::*;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = 8 + Offer::INIT_SPACE,
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> MakeOffer<'info> {
    pub fn make_offer(&mut self, id: u64, offer_amount: u64, wanted_amount: u64) -> Result<()> {
        self.offer.set_inner(Offer {
            id,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            wanted_amount,
            bump: self.offer.bump,
        });

        let transfer_cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            TransferChecked {
                authority: self.maker.to_account_info(),
                from: self.maker_token_account_a.to_account_info(),
                to: self.vault.to_account_info(),
                mint: self.mint_a.to_account_info(),
            },
        );

        transfer_checked(transfer_cpi_ctx, offer_amount, self.mint_a.decimals)?;

        msg!("Offer was made and the offered tokens were sent to vault");
        Ok(())
    }
}
