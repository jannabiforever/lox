use std::fmt;

use crate::{parse::ParseError, tokenize::TokenizeError};

#[derive(Debug, Clone, thiserror::Error)]
pub enum LoxError {
    #[error("{0}")]
    Tokenize(TokenizeError),

    #[error("{0}")]
    Parse(ParseError),
}

impl From<TokenizeError> for LoxError {
    fn from(value: TokenizeError) -> Self {
        Self::Tokenize(value)
    }
}

impl From<ParseError> for LoxError {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

/// A wrapper for a value that also contains the line number where the value was found.
/// This is used for error reporting.
#[derive(Debug)]
pub struct WithLine<T> {
    pub line: usize,
    pub value: T,
}

impl<T> WithLine<T> {
    pub fn new(line: usize, value: T) -> Self {
        Self { line, value }
    }

    pub fn split(self) -> (usize, T) {
        (self.line, self.value)
    }

    pub fn inner_ref(&self) -> &T {
        &self.value
    }

    pub fn into_inner(self) -> T {
        self.value
    }

    pub fn map<F: FnOnce(T) -> U, U>(self, f: F) -> WithLine<U> {
        WithLine::new(self.line, f(self.value))
    }
}

/// A wrapper for a [`Result`] that also contains the line number where the value was found.
/// This is used for error reporting.
pub type ResultWithLine<T, E> = WithLine<std::result::Result<T, E>>;

impl<T, E> ResultWithLine<T, E> {
    /// As demanded by main.rs, we need to cast [`ResultWithLine<T, E>`] to [`Result<T, WithLine<E>>`].
    pub fn exposure(self) -> Result<T, WithLine<E>> {
        self.value.map_err(|err| WithLine::new(self.line, err))
    }

    pub fn cast_down(self) -> Result<WithLine<T>, WithLine<E>> {
        let (line, value) = self.split();
        match value {
            Ok(value) => Ok(WithLine::new(line, value)),
            Err(err) => Err(WithLine::new(line, err)),
        }
    }
}

impl fmt::Display for WithLine<LoxError> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] {}", self.line, self.value)
    }
}

/// A trait for reporting errors.
/// Every parser-like struct should implement this trait.
pub trait ErrorReporter<E: Into<LoxError>> {
    /// Current line number.
    /// Used for checking where the error occurred.
    fn line(&self) -> usize;

    fn wrap<T>(&self, value: T) -> WithLine<T> {
        WithLine::new(self.line(), value)
    }
}
