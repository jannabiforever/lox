use std::{cell::RefCell, io::Write, rc::Rc};

use super::{RuntimeError, StmtAst, StmtParseError, StmtParser};
use crate::{env::Runnable, expr::ExprAst, literal::LoxValue, mac::tt, Env, Evaluatable};

#[derive(Debug, Clone, PartialEq)]
pub struct If<'a> {
    condition: ExprAst<'a>,
    body: Box<StmtAst<'a>>,
    else_body: Option<Box<StmtAst<'a>>>,
}

impl Runnable for If<'_> {
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<Option<LoxValue>, RuntimeError> {
        let If {
            condition,
            body,
            else_body,
        } = self;

        let condition_value = condition.eval(env.clone())?;

        if condition_value.is_literal_and(|l| l.is_truthy()) {
            if let Some(value) = body.run(env.clone())? {
                return Ok(Some(value));
            }
        } else if let Some(else_body) = else_body {
            if let Some(value) = else_body.run(env.clone())? {
                return Ok(Some(value));
            }
        }

        Ok(None)
    }
}

impl<'a> StmtParser<'a, '_> {
    pub(super) fn parse_if(&mut self) -> Result<If<'a>, StmtParseError> {
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
