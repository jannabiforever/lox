use std::{cell::RefCell, io::Write, rc::Rc};

use super::{RuntimeError, StmtAst, StmtParseError, StmtParser};
use crate::{env::Runnable, error::LoxError, expr::ExprAst, literal::LoxValue, Env, Evaluatable};

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Debug, Clone, PartialEq)]
pub struct While<'src> {
    condition: ExprAst<'src>,
    body: Box<StmtAst<'src>>,
}

impl<'src> Runnable<'src> for While<'src> {
    fn run<W: Write>(
        &self,
        env: Rc<RefCell<Env<'src>>>,
        stdout: &mut W,
    ) -> Result<Option<LoxValue<'src>>, LoxError<RuntimeError>> {
        let While { condition, body } = self;

        while condition
            .eval(env.clone(), stdout)?
            .is_literal_and(|l| l.is_truthy())
        {
            if let Some(value) = body.run(env.clone(), stdout)? {
                return Ok(Some(value));
            }
        }

        Ok(None)
    }

    fn line(&self) -> usize {
        self.body.line()
    }
}

impl<'src> StmtParser<'src, '_> {
    pub(super) fn parse_while(&mut self) -> Result<While<'src>, StmtParseError> {
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
