use std::process::ExitCode;

use crate::error::IntoLoxError;

#[derive(Debug, Clone, thiserror::Error)]
pub enum TokenizeError {
    #[error("Error: Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Error: Unterminated string.")]
    UnterminatedString,
}

impl IntoLoxError for TokenizeError {
    fn exit_code(&self) -> ExitCode {
        ExitCode::from(65)
    }
}
