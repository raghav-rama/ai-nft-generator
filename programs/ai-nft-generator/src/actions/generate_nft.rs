use crate::*;
use switchboard_solana::{FunctionAccountData, FunctionRequestAccountData};

#[derive(Accounts)]
pub struct GenerateNft<'info> {
    #[account(
        mut, 
        seeds = [ORACLE_SEED],
        bump = oracle.load()?.bump
    )]
    pub oracle: AccountLoader<'info, MyOracleState>,
    #[account(
        constraint = function.load()?.validate(
            &enclave_signer.to_account_info()
        )? @ AiNftGenerateError::FunctionValidationFailed
    )]
    pub function: AccountLoader<'info, FunctionAccountData>,
    #[account(
        constraint = request.validate_signer(
            &function.to_account_info(),
            &enclave_signer.to_account_info(),
        )? @ AiNftGenerateError::FunctionValidationFailed
    )]
    pub request: Box<Account<'info, FunctionRequestAccountData>>,
    pub enclave_signer: Signer<'info>
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct GenerateNftParams {
    pub nft: NftMetadataBorsh,
}


impl GenerateNft<'_> {
    pub fn validate(&self, _ctx: &Context<GenerateNft>, _params: &GenerateNftParams) -> Result<()> {
        Ok(())
    }
    pub fn actuate(ctx: Context<Self>, params: GenerateNftParams) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle.load_mut()?;
        msg!("saving nft metadata");
        oracle.update_nft_metadata(params.nft)?;
        Ok(())
    }
}
