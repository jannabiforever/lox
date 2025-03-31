use std::fmt;

use crate::{evaluate::EvaluateError, parse::ParseError, tokenize::TokenizeError};

#[derive(Debug, thiserror::Error)]
pub(crate) struct LoxError {
    pub(crate) line: usize,
    pub(crate) kind: LoxErrorKind,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] {}", self.line, self.kind)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum LoxErrorKind {
    #[error("{0}")]
    Tokenize(TokenizeError),

    #[error("{0}")]
    Parse(ParseError),

    #[error("{0}")]
    Evaluate(EvaluateError),
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

pub trait IntoLoxError: Sized {
    fn error_kind(self) -> LoxErrorKind;

    fn error(self, line: usize) -> LoxError {
        LoxError {
            line,
            kind: self.error_kind(),
        }
    }
}
