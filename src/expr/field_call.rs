use std::fmt;

use super::{
    ExprAst,
    ExprParseError::{self, *},
    ExprParser,
};
use crate::mac::tt;

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldCall<'a> {
    pub object: Box<ExprAst<'a>>,
    pub field: String,
}

impl fmt::Display for FieldCall<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.object, self.field)
    }
}

impl<'a> ExprParser<'a, '_> {
    pub(super) fn parse_field_call(
        &mut self,
        left: ExprAst<'a>,
    ) -> Result<FieldCall<'a>, ExprParseError> {
        self.token_stream.next();
        let field = self
            .token_stream
            .expect(tt!("identifier"))
            .map_err(|unexpected_token| ExpectedFieldName(unexpected_token.src.to_string()))?
            .src
            .to_string();

        Ok(FieldCall {
            object: Box::new(left),
            field,
        })
    }
}
