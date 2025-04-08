use std::process::ExitCode;

use crate::{env::EvaluateError, error::IntoLoxError, mac::impl_from, parse::ExprParseError};

#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum RuntimeError {
    #[error("{0}")]
    EvaluateError(EvaluateError),

    #[error("Error: Cannot assign value into '{0}'.")]
    InvalidAssignmentTarget(String),
}

impl_from!(RuntimeError: EvaluateError);

impl IntoLoxError for RuntimeError {
    fn exit_code(&self) -> ExitCode {
        ExitCode::from(70)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum StmtParseError {
    #[error("{0}")]
    ExprParseError(ExprParseError),

    #[error("Expected semicolon, but found {0}")]
    ExpectedSemicolon(String),

    #[error("Expected assign or variable after 'var', but got {0}")]
    InvalidVarDecl(String),

    #[error("Error: Expected '}}', but got {0}")]
    ExpectedEndOfBracket(String),

    #[error("Error: Expected '(', but got {0}")]
    ExpectedOpeningParentheses(String),

    #[error("Error: Expected ')', but got {0}")]
    ExpectedClosingParentheses(String),

    #[error("Error: {0} cannot be for statement's initializer.")]
    InvalidForStmtInitializer(String),

    #[error("Error: {0} cannot be for statement's body.")]
    InvalidForStmtBody(String),

    #[error("Error: {0} cannot be while statement's body")]
    InvalidWhileStmtBody(String),
}

impl_from!(StmtParseError: ExprParseError);

impl IntoLoxError for StmtParseError {
    fn exit_code(&self) -> ExitCode {
        ExitCode::from(65)
    }
}
