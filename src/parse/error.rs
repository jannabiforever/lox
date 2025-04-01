use crate::error::IntoLoxError;

#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum ExprParseError {
    #[error("Error at end: Expected ')'.")]
    ExpectedClosingParenthesis,

    #[error("Error at '{0}': Expected expression.")]
    ExpectedExpression(String),

    #[error("Error at '{0}': Field name must be an identifier.")]
    ExpectedFieldName(String),

    #[error("Error at '{0}': Invalid function argument.")]
    InvalidFunctionArgument(String),
}

impl IntoLoxError for ExprParseError {
    fn error_kind(self) -> crate::LoxErrorKind {
        crate::LoxErrorKind::ExprParse(self)
    }
}
