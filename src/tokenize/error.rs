#[derive(Debug, Clone, thiserror::Error)]
pub enum TokenizeError {
    #[error("Error: Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Error: Unterminated string.")]
    UnterminatedString,
}
