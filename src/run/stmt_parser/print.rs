use crate::{
    run::{error::StmtParseError, stmt_ast::Print},
    tokenize::tt,
};

use super::StmtParser;

impl<'a> StmtParser<'a, '_> {
    pub fn parse_print(&mut self) -> Result<Print, StmtParseError> {
        self.token_stream.next(); // consume the 'print' token.
        let expr = self.parse_following_expression()?;

        match self.token_stream.expect(tt!(";")) {
            Ok(_) => Ok(Print { expr }),
            Err(unexpected_token) => Err(StmtParseError::ExpectedSemicolon(
                unexpected_token.src.to_string(),
            )),
        }
    }
}
