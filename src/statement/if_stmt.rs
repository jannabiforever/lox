use std::{cell::RefCell, io::Write, rc::Rc};

use crate::{env::Runnable, expr::ExprAst, mac::tt, Env, Evaluatable};

use super::{RuntimeError, StmtAst, StmtParseError, StmtParser};

#[derive(Debug, Clone, PartialEq)]
pub struct If {
    condition: ExprAst,
    body: Box<StmtAst>,
    else_body: Option<Box<StmtAst>>,
}

impl Runnable for If {
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<(), RuntimeError> {
        let If {
            condition,
            body,
            else_body,
        } = self;

        let condition_value = condition.eval(env.clone())?;

        if condition_value.is_literal_and(|l| l.is_truthy()) {
            body.run(env.clone())?;
        } else if let Some(else_body) = else_body {
            else_body.run(env.clone())?;
        }

        Ok(())
    }
}

impl StmtParser<'_, '_> {
    pub(super) fn parse_if(&mut self) -> Result<If, StmtParseError> {
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
