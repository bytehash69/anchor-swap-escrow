use anchor_lang::prelude::*;

declare_id!("25rZpGqeAjhSF2deWQV2B5n7orxhaqyXuhvt822JmfYL");

pub mod state;
pub use state::*;

#[program]
pub mod anchor_swap_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
