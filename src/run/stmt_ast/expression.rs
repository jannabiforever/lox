use crate::parse::ExprAst;

#[derive(Debug, Clone)]
pub(crate) struct Expression {
    pub(crate) expr: ExprAst,
}
