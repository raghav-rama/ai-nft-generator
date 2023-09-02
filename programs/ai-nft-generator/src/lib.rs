use anchor_lang::prelude::*;

pub mod model;
pub use model::*;

pub mod actions;
pub use actions::*;

declare_id!("9zFjUs3LhY39t7HbJWMBFt6VrBMktVzbvBXAjFo7V8yS");

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

    // #[access_control(ctx.accounts.validate(&ctx, &params))]
    // pub fn refresh_oracles(
    //     ctx: Context<NftMetadata>,
    //     params: NftMetadataBorsh,
    // ) -> anchor_lang::Result<()> {
    //     NftMetadata::actuate(&ctx, &params)
    // }
}
