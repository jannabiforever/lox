use std::{cell::RefCell, io::Write, rc::Rc};

use super::{RuntimeError, StmtParser};
use crate::{
    env::Runnable, error::LoxError, expr::ExprAst, literal::LoxValue,
    statement::error::StmtParseError, Env, Evaluatable,
};

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Print<'src> {
    pub(crate) expr: ExprAst<'src>,
}

impl<'src> Runnable<'src> for Print<'src> {
    fn run<W: Write>(
        &self,
        env: Rc<RefCell<Env<'src>>>,
        stdout: &mut W,
    ) -> Result<Option<LoxValue<'src>>, LoxError<RuntimeError>> {
        let value = self.expr.eval(env.clone(), stdout)?;
        writeln!(stdout, "{value}").unwrap();
        Ok(None)
    }

    fn line(&self) -> usize {
        self.expr.line()
    }
}

impl<'src> StmtParser<'src, '_> {
    pub(super) fn parse_print(&mut self) -> Result<Print<'src>, StmtParseError> {
        self.token_stream.next(); // consume the 'print' token.
        let expr = self.parse_following_expression()?;
        self.expect_semicolon()?;

        Ok(Print { expr })
    }
}
