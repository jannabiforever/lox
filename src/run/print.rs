use crate::{parse::ExprAst, run::error::StmtParseError};

use super::{Runtime, RuntimeError, StmtParser};

impl StmtParser<'_, '_> {
    pub fn parse_print(&mut self) -> Result<Print, StmtParseError> {
        self.token_stream.next(); // consume the 'print' token.
        let expr = self.parse_following_expression()?;
        self.expect_semicolon()?;

        Ok(Print { expr })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Print {
    pub(crate) expr: ExprAst,
}

impl Runtime {
    pub fn run_print(&self, print: Print) -> Result<(), RuntimeError> {
        let value = self.evaluate(&print.expr)?;
        println!("{}", value.pretty());
        Ok(())
    }
}
