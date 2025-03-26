use std::fmt;

use crate::{parse::ParseError, tokenize::TokenizeError};

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

    #[error("{0}")]
    Parse(ParseError),
}

impl From<TokenizeError> for LoxErrorKind {
    fn from(value: TokenizeError) -> Self {
        Self::Tokenize(value)
    }
}

impl From<ParseError> for LoxErrorKind {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}
