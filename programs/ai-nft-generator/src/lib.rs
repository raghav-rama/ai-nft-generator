use anchor_lang::prelude::*;

pub mod model;
pub use model::*;

declare_id!("ATDso9o279ksvov6rAm4MoXQrxFPQX1q5qiX4yXQLnxh");

#[program]
pub mod ai_nft_generator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
