use crate::error::IntoLoxError;

#[derive(Debug, thiserror::Error, Clone)]
pub(crate) enum EvaluateError {
    #[error("Error: Operand must be {0}")]
    OperandMustBe(&'static str),

    #[error("Error: Undefined variable '{0}'.")]
    UndefinedVariable(String),

    #[error("Error: Cannot assign value into '{0}'.")]
    InvalidAssignmentTarget(String),
}

impl IntoLoxError for EvaluateError {
    fn error_kind(self) -> crate::LoxErrorKind {
        crate::LoxErrorKind::Evaluate(self)
    }
}
