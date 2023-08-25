use crate::*;

#[derive(Accounts)]
#[instruction(params: InitializeParams)]
pub struct Initialize<'info> {
    #[account(
        init,
        space = 8,
        payer = payer,
        seeds = [PROGRAM_SEED],
        bump
    )]
    pub program: AccountLoader<'info, MyProgramState>,
    #[account(
        init,
        space = 8,
        payer = payer,
        seeds = [ORACLE_SEED],
        bump
    )]
    pub oracle: AccountLoader<'info, MyOracleState>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeParams {
    pub authority: Pubkey,
    pub function: Pubkey,
}