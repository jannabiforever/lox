use std::io::Write;

use crate::parse::ExprAst;

use super::{Runtime, RuntimeError, StmtParseError, StmtParser};

#[derive(Debug, Clone)]
pub(crate) struct Expression {
    pub(crate) expr: ExprAst,
}

impl StmtParser<'_, '_> {
    pub(super) fn parse_expression_stmt(&mut self) -> Result<Expression, StmtParseError> {
        let expr = self.parse_following_expression()?;
        self.expect_semicolon()?;
        Ok(Expression { expr })
    }
}

impl<W: Write> Runtime<W> {
    pub(super) fn run_expression(&self, expr: Expression) -> Result<(), RuntimeError> {
        self.evaluate(&expr.expr).map(|_| ())
    }
}
