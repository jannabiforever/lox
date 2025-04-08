use std::{fmt, io::Write, process::ExitCode};

pub(crate) trait LoxResulT {
    fn write_to_buffer<W1: Write, W2: Write>(
        self,
        ok_buf: &mut W1,
        err_buf: &mut W2,
    ) -> Result<(), ExitCode>;
}

impl<T: fmt::Display, E: IntoLoxError> LoxResulT for Result<T, LoxError<E>> {
    fn write_to_buffer<W1: Write, W2: Write>(
        self,
        ok_buf: &mut W1,
        err_buf: &mut W2,
    ) -> Result<(), ExitCode> {
        match self {
            Ok(result) => {
                writeln!(ok_buf, "{result}").unwrap();
                Ok(())
            }
            Err(err) => {
                writeln!(err_buf, "{err}").unwrap();
                Err(err.kind.exit_code())
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) struct LoxError<E: IntoLoxError> {
    pub(crate) line: usize,
    pub(crate) kind: E,
}

impl<E: IntoLoxError> fmt::Display for LoxError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] {}", self.line, self.kind)
    }
}

pub trait IntoLoxError: Sized + std::error::Error {
    // Required method
    fn exit_code(&self) -> ExitCode;

    // Provided method
    fn error(self, line: usize) -> LoxError<Self> {
        LoxError { line, kind: self }
    }
}
