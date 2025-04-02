mod expression;
mod print;
mod var_decl;

pub(super) use expression::Expression;
pub(super) use print::Print;
pub(super) use var_decl::VarDecl;

#[derive(Debug, Clone)]
pub(crate) enum StmtAst {
    Expression(Expression),
    Print(Print),
    VarDecl(VarDecl),
}

impl From<Expression> for StmtAst {
    fn from(expression: Expression) -> Self {
        StmtAst::Expression(expression)
    }
}

impl From<Print> for StmtAst {
    fn from(print: Print) -> Self {
        StmtAst::Print(print)
    }
}

impl From<VarDecl> for StmtAst {
    fn from(var_decl: VarDecl) -> Self {
        StmtAst::VarDecl(var_decl)
    }
}
