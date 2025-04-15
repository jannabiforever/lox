use std::{cell::RefCell, io::Write, rc::Rc};

use super::{StmtParseError, StmtParser};
use crate::{
    env::RuntimeError, expr::ExprAst, literal::LoxValue, mac::tt, Env, Evaluatable, Runnable,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Return<'a> {
    pub(crate) expr: Option<ExprAst<'a>>,
    /// return token's line, but not directly used.
    line: usize,
}

impl Runnable for Return<'_> {
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<Option<LoxValue>, RuntimeError> {
        if env.borrow().is_global() {
            return Err(RuntimeError::ReturnAtGlobal);
        }

        let value = self
            .expr
            .as_ref()
            .map(|expr| expr.eval(env.clone()))
            .transpose()?
            .unwrap_or_default();

        Ok(Some(value))
    }

    fn line(&self) -> usize {
        if let Some(expr) = self.expr.as_ref() {
            // when expr presented, get its line.
            expr.line()
        } else {
            // else, get return token's line.
            self.line
        }
    }
}

impl<'a> StmtParser<'a, '_> {
    pub(super) fn parse_return(&mut self) -> Result<Return<'a>, StmtParseError> {
        let line = self.token_stream.next().line; // Consume 'return'.
        let expr = if self.token_stream.peek().token_type != tt!(";") {
            Some(self.parse_following_expression()?)
        } else {
            None
        };
        self.expect_semicolon()?;
        Ok(Return { expr, line })
    }
}
