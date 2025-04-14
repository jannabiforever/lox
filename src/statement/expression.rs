use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use super::RuntimeError;
use super::StmtParseError;
use super::StmtParser;
use crate::env::Runnable;
use crate::expr::ExprAst;
use crate::literal::LoxValue;
use crate::Env;
use crate::Evaluatable;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Expression {
    pub(crate) expr: ExprAst,
}

impl Runnable for Expression {
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<Option<LoxValue>, RuntimeError> {
        self.expr.eval(env)?;
        Ok(None)
    }
}

impl StmtParser<'_, '_> {
    pub(super) fn parse_expression_stmt(&mut self) -> Result<Expression, StmtParseError> {
        let expr = self.parse_following_expression()?;
        self.expect_semicolon()?;
        Ok(Expression { expr })
    }
}
