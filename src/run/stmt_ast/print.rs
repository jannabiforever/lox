use crate::parse::ExprAst;

#[derive(Debug, Clone)]
pub(crate) struct Print {
    pub(crate) expr: ExprAst,
}
