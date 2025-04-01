use crate::{
    parse::{expr_ast::Grouping, ExprParseError},
    tokenize::tt,
};

impl super::ExprParser<'_, '_> {
    /// Parse a grouping expression follwing only if exists.
    /// And consume from '(' to ')'.
    pub(super) fn parse_grouping(&mut self) -> Option<Result<Grouping, ExprParseError>> {
        match self.token_stream.peek().token_type {
            tt!("(") => {
                // Consume '('.
                self.token_stream.next();

                let inner = match self.parse() {
                    Ok(inner) => Box::new(inner),
                    Err(e) => return Some(Err(e)),
                };

                if self.token_stream.expect(tt!(")")).is_ok() {
                    Some(Ok(Grouping { inner }))
                } else {
                    Some(Err(ExprParseError::ExpectedClosingParenthesis))
                }
            }
            _ => None,
        }
    }
}
