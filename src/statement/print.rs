use std::{cell::RefCell, io::Write, rc::Rc};

use super::{RuntimeError, StmtParser};
use crate::{
    env::Runnable, expr::ExprAst, literal::LoxValue, statement::error::StmtParseError, Env,
    Evaluatable,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Print<'a> {
    pub(crate) expr: ExprAst<'a>,
}

impl Runnable for Print<'_> {
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<Option<LoxValue>, RuntimeError> {
        let value = self.expr.eval(env.clone())?;
        writeln!(env.borrow().stdout.borrow_mut(), "{value}").unwrap();
        Ok(None)
    }
}

impl<'a> StmtParser<'a, '_> {
    pub(super) fn parse_print(&mut self) -> Result<Print<'a>, StmtParseError> {
        self.token_stream.next(); // consume the 'print' token.
        let expr = self.parse_following_expression()?;
        self.expect_semicolon()?;

        Ok(Print { expr })
    }
}
