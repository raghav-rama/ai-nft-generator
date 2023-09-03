use crate::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum AiNftGenerateError {
    #[msg("Function validation failed")]
    FunctionValidationFailed,
    #[msg("Function execution failed")]
    FunctionExecutionFailed,
}