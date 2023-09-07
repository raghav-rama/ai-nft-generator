use anchor_lang::prelude::*;

pub mod model;
pub use model::*;

pub mod actions;
pub use actions::*;

pub mod error;
pub use error::*;

declare_id!("CvPJsEqiyf53UA1mnFq9Uib6Yy29MVJRgketa81Fx8tG");

pub const PROGRAM_SEED: &[u8] = b"ai-nft-generator";

pub const ORACLE_SEED: &[u8] = b"ai-nft-generator-oracle";

#[program]
pub mod ai_nft_generator {
    use super::*;

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn initialize(
        ctx: Context<Initialize>,
        params: InitializeParams,
    ) -> anchor_lang::Result<()> {
        Initialize::actuate(&ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn mint_ai_nft(
        ctx: Context<GenerateNft>,
        params: GenerateNftParams,
    ) -> anchor_lang::Result<()> {
        GenerateNft::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn set_function(
        ctx: Context<SetFunction>,
        params: SetFunctionParams,
    ) -> anchor_lang::Result<()> {
        SetFunction::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn trigger_function(
        ctx: Context<TriggerFunction>,
        params: TriggerFunctionParams,
    ) -> anchor_lang::Result<()> {
        TriggerFunction::actuate(&ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn set_user_prompt(
        ctx: Context<UserPrompt>,
        params: UserPromptParams,
    ) -> anchor_lang::Result<()> {
        UserPrompt::actuate(ctx, params)
    }
}
