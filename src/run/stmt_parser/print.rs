use crate::run::{error::StmtParseError, stmt_ast::Print};

use super::StmtParser;

impl StmtParser<'_, '_> {
    pub fn parse_print(&mut self) -> Result<Print, StmtParseError> {
        self.token_stream.next(); // consume the 'print' token.
        let expr = self.parse_following_expression()?;
        self.expect_semicolon()?;

        Ok(Print { expr })
    }
}
