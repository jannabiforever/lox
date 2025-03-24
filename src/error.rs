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
pub enum ASTError {
    #[error("Error: Expect binary operator.")]
    ExpectedBinaryOperator,

    #[error("Error at end: Expect '{0}'.")]
    ExpectClosingDelimiter(char),

    /// Inner value is the expected token.
    #[error("Error: Expect '{0}'.")]
    ExpectedToken(String),

    #[error("Error: Expect expression.")]
    ExpectedExpression,

    #[error("Error: Invalid assignment target.")]
    InvalidAssignmentTarget,
}

impl ASTError {
    #[allow(dead_code)]
    pub fn exit_code(&self) -> ExitCode {
        ExitCode::from(65)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    /// Inner value is the operand type.
    #[error("Error: Operand must be {0}")]
    UnaryOperandType(&'static str),

    /// Inner value is the operand type.
    #[error("Error: Operands must be {0}.")]
    BinaryOperandType(&'static str),

    #[error("Error at 'return': Can't return from top-level code.")]
    ReturnFromTopLevel,

    /// Inner value is the variable name.
    #[error("Undefined variable '{0}'.")]
    UndefinedVar(String),
}

impl RuntimeError {
    pub fn exit_code(&self) -> ExitCode {
        ExitCode::from(70)
    }
}

pub struct WithLine<T> {
    pub line: usize,
    inner: T,
}

impl<T> WithLine<T> {
    pub fn new(line: usize, inner: T) -> Self {
        Self { line, inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn as_ref(&self) -> WithLine<&T> {
        WithLine {
            line: self.line,
            inner: &self.inner,
        }
    }

    pub fn map<F: FnOnce(T) -> U, U>(self, f: F) -> WithLine<U> {
        WithLine {
            line: self.line,
            inner: f(self.inner),
        }
    }

    pub fn split(self) -> (usize, T) {
        (self.line, self.inner)
    }
}

impl<T> WithLine<Result<T, LoxError>> {
    /// Lox tracks the current line number only for error reporting.
    /// So when it comes to the real usage, it should be casted to Result<T, WithLine<LoxError>>.
    pub fn cast_down(self) -> Result<T, WithLine<LoxError>> {
        match self {
            WithLine {
                inner: Ok(value), ..
            } => Ok(value),
            WithLine {
                line,
                inner: Err(value),
            } => Err(WithLine { line, inner: value }),
        }
    }
}

impl<T> From<WithLine<Result<T, LoxError>>> for Result<WithLine<T>, WithLine<LoxError>> {
    fn from(value: WithLine<Result<T, LoxError>>) -> Self {
        match value {
            WithLine {
                line,
                inner: Ok(value),
            } => Ok(WithLine { line, inner: value }),
            WithLine {
                line,
                inner: Err(value),
            } => Err(WithLine { line, inner: value }),
        }
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

impl std::fmt::Display for WithLine<LoxError> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] {}", self.line, self.inner)
    }
}

pub(crate) trait ErrorReporter<E: Into<LoxError>> {
    fn line(&self) -> usize;

    fn wrap<T>(&self, value: T) -> WithLine<T> {
        WithLine {
            line: self.line(),
            inner: value,
        }
    }

    fn get_lox_ok<T>(&self, value: T) -> WithLine<Result<T, LoxError>> {
        self.wrap(Ok(value))
    }

    fn get_lox_err<T>(&self, error: E) -> WithLine<Result<T, LoxError>> {
        self.wrap(Err(error.into()))
    }
}
