#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum ParseError {
    #[error("Error at end: Expected ')'.")]
    ExpectedClosingParenthesis,

    #[error("Error at '{0}': Expected expression.")]
    ExpectedExpression(String),
}
