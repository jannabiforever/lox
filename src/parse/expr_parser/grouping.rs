use crate::{
    parse::{expr_ast::Grouping, ParseError},
    tokenize::tt,
};

impl super::ExprParser<'_, '_> {
    /// Parse a grouping expression follwing only if exists.
    /// And consume from '(' to ')'.
    pub(super) fn parse_grouping(&mut self) -> Option<Result<Grouping, ParseError>> {
        match self.peek().token_type {
            tt!("(") => {
                // Consume '('.
                self.next();

                let inner = match self.parse() {
                    Ok(inner) => Box::new(inner),
                    Err(e) => return Some(Err(e)),
                };

                if self.expect(tt!(")")).is_ok() {
                    Some(Ok(Grouping { inner }))
                } else {
                    Some(Err(ParseError::ExpectedClosingParenthesis))
                }
            }
            _ => None,
        }
    }
}
