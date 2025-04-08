use std::io::Write;

use crate::{mac::tt, parse::ExprAst};

use super::{Runtime, RuntimeError, StmtAst, StmtParseError, StmtParser};

#[derive(Debug, Clone)]
pub struct If {
    condition: ExprAst,
    body: Box<StmtAst>,
    else_body: Option<Box<StmtAst>>,
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

impl<W: Write> Runtime<W> {
    pub(super) fn run_if(&self, if_stmt: If) -> Result<(), RuntimeError> {
        let If {
            condition,
            body,
            else_body,
        } = if_stmt;

        let condition_value = self.evaluate(&condition)?;

        if condition_value.is_truthy() {
            self.run(*body)?;
        } else if let Some(else_body) = else_body {
            self.run(*else_body)?;
        }

        Ok(())
    }
}
