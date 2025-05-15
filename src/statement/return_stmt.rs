use std::{cell::RefCell, io::Write, rc::Rc};

use super::{StmtParseError, StmtParser};
use crate::{
    env::RuntimeError,
    error::{IntoLoxError, LoxError},
    expr::ExprAst,
    literal::LoxValue,
    mac::tt,
    Env, Evaluatable, Runnable,
};

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Return<'src> {
    pub(crate) expr: Option<ExprAst<'src>>,
    /// return token's line, but not directly used.
    line: usize,
}

impl<'src> Runnable<'src> for Return<'src> {
    fn run<W: Write>(
        &self,
        env: Rc<RefCell<Env<'src>>>,
        stdout: &mut W,
    ) -> Result<Option<LoxValue<'src>>, LoxError<RuntimeError>> {
        if env.borrow().is_global() {
            return Err(RuntimeError::ReturnAtGlobal.at(self.line()));
        }

        let value = self
            .expr
            .as_ref()
            .map(|expr| expr.eval(env.clone(), stdout))
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

impl<'src> StmtParser<'src, '_> {
    pub(super) fn parse_return(&mut self) -> Result<Return<'src>, StmtParseError> {
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
