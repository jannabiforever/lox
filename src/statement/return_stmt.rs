use std::{cell::RefCell, io::Write, rc::Rc};

use super::{StmtParseError, StmtParser};
use crate::{
    env::RuntimeError, expr::ExprAst, literal::LoxValue, mac::tt, Env, Evaluatable, Runnable,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Return<'a> {
    pub(crate) expr: Option<ExprAst<'a>>,
}

impl Runnable for Return<'_> {
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
