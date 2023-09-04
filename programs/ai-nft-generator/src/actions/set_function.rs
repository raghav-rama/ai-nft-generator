use switchboard_solana::FunctionAccountData;

use crate::*;

#[derive(Accounts)]
#[instruction(params: SetFunctionParams)]
pub struct SetFunction<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_SEED],
        bump = program.load()?.bump,
        has_one = authority,
    )]
    pub program: AccountLoader<'info, MyProgramState>,
    pub function: AccountLoader<'info, FunctionAccountData>,
    pub authority: Signer<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SetFunctionParams {}

impl SetFunction<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &SetFunctionParams) -> Result<()> {
        Ok(())
    }
    pub fn actuate(ctx: Context<Self>, _params: SetFunctionParams) -> Result<()> {
        let program = &mut ctx.accounts.program.load_mut()?;
        program.function = *ctx.accounts.function.to_account_info().key;
        Ok(())
    }
}
