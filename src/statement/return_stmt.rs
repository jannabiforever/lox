use crate::expr::ExprAst;

use super::{StmtParseError, StmtParser};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Return {
    pub(crate) inner: ExprAst,
}

impl StmtParser<'_, '_> {
    pub(super) fn parse_return(&mut self) -> Result<Return, StmtParseError> {
        self.token_stream.next(); // Consume 'return'.
        let inner = self.parse_following_expression()?;
        self.expect_semicolon()?;
        Ok(Return { inner })
    }
}
