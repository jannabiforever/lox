use std::process::ExitCode;

#[derive(Debug, thiserror::Error)]
pub enum LoxError {
    #[error("{0}")]
    Lex(LexError),

    #[error("{0}")]
    AbstractSyntaxTree(ASTError),

    #[error("{0}")]
    RuntimeError(RuntimeError),
}

impl LoxError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            LoxError::Lex(_) => ExitCode::from(65),
            LoxError::AbstractSyntaxTree(_) => ExitCode::from(65),
            LoxError::RuntimeError(_) => ExitCode::from(70),
        }
    }
}

impl From<LexError> for LoxError {
    fn from(error: LexError) -> Self {
        LoxError::Lex(error)
    }
}

impl From<ASTError> for LoxError {
    fn from(error: ASTError) -> Self {
        LoxError::AbstractSyntaxTree(error)
    }
}

impl From<RuntimeError> for LoxError {
    fn from(error: RuntimeError) -> Self {
        LoxError::RuntimeError(error)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LexError {}

#[derive(Debug, thiserror::Error)]
pub enum ASTError {}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {}
