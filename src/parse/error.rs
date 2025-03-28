#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum ParseError {
    #[error("Error at end: Expected ')'.")]
    ExpectedClosingParenthesis,
}
