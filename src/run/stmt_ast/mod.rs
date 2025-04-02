mod expression;
mod print;

pub(super) use expression::Expression;
pub(super) use print::Print;

#[derive(Debug, Clone)]
pub(crate) enum StmtAst {
    Print(Print),
    Expression(Expression),
}

impl From<Print> for StmtAst {
    fn from(print: Print) -> Self {
        StmtAst::Print(print)
    }
}

impl From<Expression> for StmtAst {
    fn from(expression: Expression) -> Self {
        StmtAst::Expression(expression)
    }
}
