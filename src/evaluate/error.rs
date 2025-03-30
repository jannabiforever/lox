#[derive(Debug, thiserror::Error)]
pub(crate) enum EvaluateError {
    #[error("Error: Operand must be {0}")]
    OperandMustBe(&'static str),
}
