use std::fmt;

use crate::mac::tt;

use super::{ExprAst, ExprParseError, ExprParser};

#[derive(Debug, Clone)]
pub struct FieldCall {
    pub object: Box<ExprAst>,
    pub field: String,
}

impl fmt::Display for FieldCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.object, self.field)
    }
}

impl ExprParser<'_, '_> {
    pub(super) fn parse_field_call(&mut self, left: ExprAst) -> Result<FieldCall, ExprParseError> {
        self.token_stream.next();
        let field = self
            .token_stream
            .expect(tt!("identifier"))
            .map_err(|unexpected_token| {
                ExprParseError::ExpectedFieldName(unexpected_token.src.to_string())
            })?
            .src
            .to_string();

        Ok(FieldCall {
            object: Box::new(left),
            field,
        })
    }
}
