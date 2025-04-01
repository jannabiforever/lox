use crate::{
    parse::{
        expr_ast::{ExprAst, FieldCall},
        ExprParseError,
    },
    tokenize::tt,
};

use super::ExprParser;

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
