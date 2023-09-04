use anchor_spl::token::spl_token::native_mint::ID as NATIVE_SPL_MINT_ID;
use switchboard_solana::{
    AssociatedToken, AttestationProgramState, AttestationQueueAccountData, FunctionAccountData,
    FunctionRequestInitAndTrigger, Mint, Token, STATE_SEED, SWITCHBOARD_ATTESTATION_PROGRAM_ID,
};

use crate::*;

#[derive(Accounts)]
#[instruction(params: UserPromptParams)]
pub struct UserPrompt<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_SEED],
        bump = program.load()?.bump,
        has_one = function,
    )]
    pub program: AccountLoader<'info, MyProgramState>,
    #[account(executable, address = SWITCHBOARD_ATTESTATION_PROGRAM_ID)]
    /// CHECK: address is explicit
    pub switchboard: AccountInfo<'info>,
    #[account(
        seeds = [STATE_SEED],
        seeds::program = switchboard.key(),
        bump = state.load()?.bump,
    )]
    pub state: AccountLoader<'info, AttestationProgramState>,
    pub attestation_queue: AccountLoader<'info, AttestationQueueAccountData>,
    #[account(
        mut,
        has_one = attestation_queue

    )]
    pub function: AccountLoader<'info, FunctionAccountData>,
    #[account(
        mut,
        signer,
        owner = system_program.key(),
        constraint = request_account.data_len() == 0 && request_account.lamports() == 0
    )]
    /// CHECK:
    pub request_account: AccountInfo<'info>,
    #[account(
        mut,
        owner = system_program.key(),
        constraint = request_account.data_len() == 0 && request_account.lamports() == 0
    )]
    /// CHECK:
    pub request_account_escrow: AccountInfo<'info>,
    #[account(address = NATIVE_SPL_MINT_ID)]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UserPromptParams {
    pub prompt: String,
}

impl UserPrompt<'_> {
    pub fn validate(&self, _ctx: &Context<UserPrompt>, _params: &UserPromptParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<UserPrompt>, params: UserPromptParams) -> Result<()> {
        let user_prompt = params.prompt.as_bytes().to_vec();
        let request_init_ctx = FunctionRequestInitAndTrigger {
            request: ctx.accounts.request_account.clone(),
            function: ctx.accounts.function.to_account_info(),
            escrow: ctx.accounts.request_account_escrow.clone(),
            mint: ctx.accounts.mint.to_account_info(),
            state: ctx.accounts.state.to_account_info(),
            attestation_queue: ctx.accounts.attestation_queue.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
        };

        request_init_ctx.invoke(
            ctx.accounts.switchboard.clone(),
            None,
            Some(1000),
            Some(512),
            Some(user_prompt),
            None,
            None,
        )?;

        Ok(())
    }
}
