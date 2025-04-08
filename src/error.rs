use std::{fmt, process::ExitCode};

use crate::{
    evaluate::EvaluateError,
    parse::ExprParseError,
    run::{RuntimeError, StmtParseError},
    tokenize::TokenizeError,
};

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
    ExprParse(ExprParseError),

    #[error("{0}")]
    Evaluate(EvaluateError),

    #[error("{0}")]
    StmtParse(StmtParseError),

    #[error("{0}")]
    Runtime(RuntimeError),
}

impl From<TokenizeError> for LoxErrorKind {
    fn from(value: TokenizeError) -> Self {
        Self::Tokenize(value)
    }
}

impl From<ExprParseError> for LoxErrorKind {
    fn from(value: ExprParseError) -> Self {
        Self::ExprParse(value)
    }
}

pub trait IntoLoxError: Sized {
    // Required method
    fn error_kind(self) -> LoxErrorKind;

    fn exit_code(&self) -> ExitCode;

    // Provided method
    fn error(self, line: usize) -> LoxError {
        LoxError {
            line,
            kind: self.error_kind(),
        }
    }
}
