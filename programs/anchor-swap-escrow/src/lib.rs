use anchor_lang::prelude::*;

declare_id!("25rZpGqeAjhSF2deWQV2B5n7orxhaqyXuhvt822JmfYL");

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

#[program]
pub mod anchor_swap_escrow {
    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        id: u64,
        offer_amount: u64,
        wanted_amount: u64,
    ) -> Result<()> {
        ctx.accounts.make_offer(id, offer_amount, wanted_amount)?;
        Ok(())
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        ctx.accounts.send_wanted_tokens_to_maker()?;
        ctx.accounts.withdraw_n_close_vault()?;
        Ok(())
    }
}
