//! Lox interpreter, always tracks the current line number for error reporting.
//! So, to clarify each utility's control flow, track line number by implementing [`ErrorReporter`] for each level's parser.
//!
//! For those which implements [`ErrorReporter`],
//! every other utils would be done without tracking lines.
//!
//! But there would be a single method that wraps all the returned value as [`WithLine<T>`].

use std::process::ExitCode;

#[derive(Debug, thiserror::Error)]
pub enum LoxError {
    #[error("{0}")]
    Lex(LexError),

    #[error("{0}")]
    AbstractSyntaxTree(ASTError),

    #[error("{0}")]
    RuntimeError(RuntimeError),
}

impl LoxError {
    #[allow(dead_code)]
    pub fn exit_code(&self) -> ExitCode {
        match self {
            LoxError::Lex(_) => ExitCode::from(65),
            LoxError::AbstractSyntaxTree(_) => ExitCode::from(65),
            LoxError::RuntimeError(_) => ExitCode::from(70),
        }
    }
}

impl From<LexError> for LoxError {
    fn from(error: LexError) -> Self {
        LoxError::Lex(error)
    }
}

impl From<ASTError> for LoxError {
    fn from(error: ASTError) -> Self {
        LoxError::AbstractSyntaxTree(error)
    }
}

impl From<RuntimeError> for LoxError {
    fn from(error: RuntimeError) -> Self {
        LoxError::RuntimeError(error)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LexError {
    #[error("Error: Unexpected character: {0}")]
    UnexpectedChar(char),

    #[error("Error: Unterminated string.")]
    UnterminatedString,
}

impl LexError {
    pub fn exit_code(&self) -> ExitCode {
        ExitCode::from(65)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ASTError {}

impl ASTError {
    #[allow(dead_code)]
    pub fn exit_code(&self) -> ExitCode {
        ExitCode::from(65)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {}

impl RuntimeError {
    #[allow(dead_code)]
    pub fn exit_code(&self) -> ExitCode {
        ExitCode::from(70)
    }
}

pub struct WithLine<T> {
    line: usize,
    inner: T,
}

impl<T> WithLine<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> From<WithLine<T>> for (usize, T) {
    fn from(value: WithLine<T>) -> Self {
        (value.line, value.inner)
    }
}

impl<T, E> From<Result<WithLine<T>, WithLine<E>>> for WithLine<Result<T, E>> {
    fn from(value: Result<WithLine<T>, WithLine<E>>) -> Self {
        match value {
            Ok(WithLine { line, inner: value }) => WithLine {
                line,
                inner: Ok(value),
            },
            Err(WithLine { line, inner: value }) => WithLine {
                line,
                inner: Err(value),
            },
        }
    }
}

pub(crate) trait ErrorReporter<E> {
    fn line(&self) -> usize;

    fn wrap<T>(&self, value: T) -> WithLine<T> {
        WithLine {
            line: self.line(),
            inner: value,
        }
    }

    fn get_ok<T>(&self, value: T) -> WithLine<Result<T, E>> {
        self.wrap(Ok(value))
    }

    fn get_err<T>(&self, error: E) -> WithLine<Result<T, E>> {
        self.wrap(Err(error))
    }
}
