use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use super::RuntimeError;
use super::StmtAst;
use super::StmtParseError;
use super::StmtParser;
use crate::env::Runnable;
use crate::expr::ExprAst;
use crate::literal::LoxValue;
use crate::Env;
use crate::Evaluatable;

#[derive(Debug, Clone, PartialEq)]
pub struct While {
    condition: ExprAst,
    body: Box<StmtAst>,
}

impl Runnable for While {
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<Option<LoxValue>, RuntimeError> {
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
}

impl StmtParser<'_, '_> {
    pub(super) fn parse_while(&mut self) -> Result<While, StmtParseError> {
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
