use std::{cell::RefCell, io::Write, rc::Rc};

use super::{RuntimeError, StmtAst, StmtParseError, StmtParser};
use crate::{
    env::Runnable, error::LoxError, expr::ExprAst, literal::LoxValue, mac::tt, Env, Evaluatable,
};

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Debug, Clone, PartialEq)]
pub struct If<'src> {
    condition: ExprAst<'src>,
    body: Box<StmtAst<'src>>,
    else_body: Option<Box<StmtAst<'src>>>,
}

impl<'src> Runnable<'src> for If<'src> {
    fn run<W: Write>(
        &self,
        env: Rc<RefCell<Env<'src>>>,
        stdout: &mut W,
    ) -> Result<Option<LoxValue<'src>>, LoxError<RuntimeError>> {
        let If {
            condition,
            body,
            else_body,
        } = self;

        let condition_value = condition.eval(env.clone(), stdout)?;

        if condition_value.is_literal_and(|l| l.is_truthy()) {
            if let Some(value) = body.run(env.clone(), stdout)? {
                return Ok(Some(value));
            }
        } else if let Some(else_body) = else_body {
            if let Some(value) = else_body.run(env.clone(), stdout)? {
                return Ok(Some(value));
            }
        }

        Ok(None)
    }

    fn line(&self) -> usize {
        if let Some(else_body) = self.else_body.as_ref() {
            else_body.line()
        } else {
            self.body.line()
        }
    }
}

impl<'src> StmtParser<'src, '_> {
    pub(super) fn parse_if(&mut self) -> Result<If<'src>, StmtParseError> {
        self.token_stream.next(); // Consume if.
        self.expect_opening_paren()?;
        let condition = self.parse_following_expression()?;
        self.expect_closing_paren()?;

        let body = Box::new(self.parse()?);
        let mut else_body = None;
        if self.token_stream.peek().token_type == tt!("else") {
            self.token_stream.next();
            else_body = Some(Box::new(self.parse()?));
        }

        Ok(If {
            condition,
            body,
            else_body,
        })
    }
}
