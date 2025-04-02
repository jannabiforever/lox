use crate::parse::ExprAst;

#[derive(Debug, Clone)]
pub(crate) struct VarDecl {
    pub(crate) var: ExprAst,
    pub(crate) value: ExprAst,
}
