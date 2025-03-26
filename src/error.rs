use std::fmt;

use crate::tokenize::TokenizeError;

#[derive(Debug, Clone, thiserror::Error)]
pub struct LoxError {
    pub line: usize,
    pub kind: LoxErrorKind,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] {}", self.line, self.kind)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum LoxErrorKind {
    #[error("{0}")]
    Tokenize(TokenizeError),
}

impl From<TokenizeError> for LoxErrorKind {
    fn from(err: TokenizeError) -> Self {
        Self::Tokenize(err)
    }
}
