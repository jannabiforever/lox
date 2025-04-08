use std::io::Write;

use crate::expr::ExprAst;

use super::{Runtime, RuntimeError, StmtAst, StmtParseError, StmtParser};

#[derive(Debug, Clone, PartialEq)]
pub struct While {
    condition: ExprAst,
    body: Box<StmtAst>,
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
            | StmtAst::For(_)) => Box::new(allowed),
            rest => return Err(StmtParseError::InvalidWhileStmtBody(format!("{rest:?}"))),
        };

        Ok(While { condition, body })
    }
}

impl<W: Write> Runtime<W> {
    pub(super) fn run_while(&self, while_stmt: While) -> Result<(), RuntimeError> {
        let While { condition, body } = while_stmt;

        while self.evaluate(&condition)?.is_truthy() {
            self.run(*body.clone())?;
        }

        Ok(())
    }
}
