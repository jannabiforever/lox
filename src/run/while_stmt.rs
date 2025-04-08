use crate::parse::ExprAst;

use super::{Runtime, RuntimeError, StmtAst, StmtParseError, StmtParser};

#[derive(Debug, Clone)]
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

        let body = Box::new(self.parse()?);

        Ok(While { condition, body })
    }
}

impl Runtime {
    pub(super) fn run_while(&self, while_stmt: While) -> Result<(), RuntimeError> {
        let While { condition, body } = while_stmt;

        while self.evaluate(&condition)?.is_truthy() {
            self.run(*body.clone())?;
        }

        Ok(())
    }
}
