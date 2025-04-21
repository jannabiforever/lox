use std::{cell::RefCell, io::Write, rc::Rc};

use super::{RuntimeError, StmtAst, StmtParseError, StmtParser};
use crate::{env::Runnable, error::LoxError, expr::ExprAst, literal::LoxValue, Env, Evaluatable};

#[derive(Debug, Clone, PartialEq)]
pub struct While<'a> {
    condition: ExprAst<'a>,
    body: Box<StmtAst<'a>>,
}

impl<'a> Runnable<'a> for While<'a> {
    fn run<W: Write>(
        &self,
        env: Rc<RefCell<Env<'a, W>>>,
    ) -> Result<Option<LoxValue<'a>>, LoxError<RuntimeError>> {
        let While { condition, body } = self;

        while condition
            .eval(env.clone())?
            .is_literal_and(|l| l.is_truthy())
        {
            if let Some(value) = body.run(env.clone())? {
                return Ok(Some(value));
            }
        }

        Ok(None)
    }

    fn line(&self) -> usize {
        self.body.line()
    }
}

impl<'a> StmtParser<'a, '_> {
    pub(super) fn parse_while(&mut self) -> Result<While<'a>, StmtParseError> {
        self.token_stream.next(); // Consume 'while'
        self.expect_opening_paren()?;
        let condition = self.parse_following_expression()?;
        self.expect_closing_paren()?;

        let body = match self.parse()? {
            allowed @ (StmtAst::Expression(_)
            | StmtAst::If(_)
            | StmtAst::Print(_)
            | StmtAst::While(_)
            | StmtAst::Block(_)
            | StmtAst::For(_)
            | StmtAst::Return(_)) => Box::new(allowed),
            rest => return Err(StmtParseError::InvalidWhileStmtBody(format!("{rest:?}"))),
        };

        Ok(While { condition, body })
    }
}
