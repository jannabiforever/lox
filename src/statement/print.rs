use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use super::RuntimeError;
use super::StmtParser;
use crate::env::Runnable;
use crate::expr::ExprAst;
use crate::literal::LoxValue;
use crate::statement::error::StmtParseError;
use crate::Env;
use crate::Evaluatable;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Print {
    pub(crate) expr: ExprAst,
}

impl Runnable for Print {
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<Option<LoxValue>, RuntimeError> {
        let value = self.expr.eval(env.clone())?;
        writeln!(env.borrow().stdout.borrow_mut(), "{}", value.pretty()).unwrap();
        Ok(None)
    }
}

impl StmtParser<'_, '_> {
    pub(super) fn parse_print(&mut self) -> Result<Print, StmtParseError> {
        self.token_stream.next(); // consume the 'print' token.
        let expr = self.parse_following_expression()?;
        self.expect_semicolon()?;

        Ok(Print { expr })
    }
}
