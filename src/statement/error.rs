use std::process::ExitCode;

use crate::{error::IntoLoxError, expr::ExprParseError, mac::impl_from};

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

    #[error("Error: Expected an identifier, but got {0}.")]
    ExpectedIdent(String),

    #[error("Error at {0}: Invalid construction for function def")]
    InvalidFunctionArgument(String),

    #[error("Error: Expected {{ for function body")]
    ExpectedBodyOfFunction,
}

impl_from!(StmtParseError: ExprParseError);

impl IntoLoxError for StmtParseError {
    fn exit_code(&self) -> ExitCode {
        ExitCode::from(65)
    }
}
