use std::{cell::RefCell, io::Write, rc::Rc};

use super::{RuntimeError, StmtParser};
use crate::{
    env::Runnable, error::LoxError, expr::ExprAst, literal::LoxValue,
    statement::error::StmtParseError, Env, Evaluatable,
};

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Print<'a> {
    pub(crate) expr: ExprAst<'a>,
}

impl<'a> Runnable<'a> for Print<'a> {
    fn run<W: Write>(
        &self,
        env: Rc<RefCell<Env<'a>>>,
        stdout: &mut W,
    ) -> Result<Option<LoxValue<'a>>, LoxError<RuntimeError>> {
        let value = self.expr.eval(env.clone(), stdout)?;
        writeln!(stdout, "{value}").unwrap();
        Ok(None)
    }

    fn line(&self) -> usize {
        self.expr.line()
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
