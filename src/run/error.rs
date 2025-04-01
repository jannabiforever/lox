use crate::{error::IntoLoxError, parse::ExprParseError};

#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum RuntimeError {}

#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum StmtParseError {
    #[error("{0}")]
    ExprParseError(ExprParseError),

    #[error("Expected semicolon, but found {0}")]
    ExpectedSemicolon(String),
}

impl From<ExprParseError> for StmtParseError {
    fn from(err: ExprParseError) -> Self {
        StmtParseError::ExprParseError(err)
    }
}

impl IntoLoxError for StmtParseError {
    fn error_kind(self) -> crate::LoxErrorKind {
        crate::LoxErrorKind::StmtParse(self)
    }
}

impl IntoLoxError for RuntimeError {
    fn error_kind(self) -> crate::LoxErrorKind {
        crate::LoxErrorKind::Runtime(self)
    }
}
