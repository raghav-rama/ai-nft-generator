use crate::*;
use switchboard_solana::attestation_program::instructions::function_trigger::FunctionTrigger;
use switchboard_solana::{
    AttestationQueueAccountData, FunctionAccountData, SWITCHBOARD_ATTESTATION_PROGRAM_ID,
};

#[derive(Accounts)]
#[instruction(params: TriggerFunctionParams)]
pub struct TriggerFunction<'info> {
    #[account(
        mut,
        has_one = authority,
        has_one = attestation_queue,
    )]
    pub function: AccountLoader<'info, FunctionAccountData>,
    #[account(
        mut, 
        signer, 
        owner = SWITCHBOARD_ATTESTATION_PROGRAM_ID, 
        constraint = request_account.data_len() != 0 && request_account.lamports() != 0
    )]
    /// CHECK:
    pub request_account: AccountInfo<'info>,
    pub attestation_queue: AccountLoader<'info, AttestationQueueAccountData>,
    pub authority: Signer<'info>,
    /// CHECK: address is explicit
    #[account(address = SWITCHBOARD_ATTESTATION_PROGRAM_ID)]
    pub atestation_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TriggerFunctionParams {}

impl TriggerFunction<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TriggerFunctionParams) -> Result<()> {
        Ok(())
    }
    pub fn actuate(ctx: &Context<Self>, _params: &TriggerFunctionParams) -> Result<()> {
        FunctionTrigger {
            function: ctx.accounts.function.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            attestation_queue: ctx.accounts.attestation_queue.to_account_info(),
        }
        .invoke(ctx.accounts.atestation_program.clone())?;
        Ok(())
    }
}
