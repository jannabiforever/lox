use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use super::StmtParseError;
use super::StmtParser;
use crate::env::RuntimeError;
use crate::expr::ExprAst;
use crate::literal::LoxValue;
use crate::mac::tt;
use crate::Env;
use crate::Evaluatable;
use crate::Runnable;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Return {
    pub(crate) expr: Option<ExprAst>,
}

impl Runnable for Return {
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<Option<LoxValue>, RuntimeError> {
        if env.borrow().is_global() {
            return Err(RuntimeError::ReturnAtGlobal);
        }
        let value = self.expr.eval(env.clone())?;
        Ok(Some(value))
    }
}

impl StmtParser<'_, '_> {
    pub(super) fn parse_return(&mut self) -> Result<Return, StmtParseError> {
        self.token_stream.next(); // Consume 'return'.
        let expr = if self.token_stream.peek().token_type != tt!(";") {
            Some(self.parse_following_expression()?)
        } else {
            None
        };
        self.expect_semicolon()?;
        Ok(Return { expr })
    }
}
