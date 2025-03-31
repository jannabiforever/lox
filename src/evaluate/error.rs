use crate::error::IntoLoxError;

#[derive(Debug, thiserror::Error, Clone)]
pub(crate) enum EvaluateError {
    #[error("Error: Operand must be {0}")]
    OperandMustBe(&'static str),
}

impl IntoLoxError for EvaluateError {
    fn error_kind(self) -> crate::LoxErrorKind {
        crate::LoxErrorKind::Evaluate(self)
    }
}
