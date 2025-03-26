use crate::{
    error::{ErrorReporter, WithLine},
    tokenize::Token,
};

use super::ParseError;

pub struct ExprParser<'a, 'b> {
    /// Note: For reporting errors, we need to know the line number.
    tokens: &'b [WithLine<Token<'a>>],
    /// Token index.
    current: usize,
}

impl ErrorReporter<ParseError> for ExprParser<'_, '_> {
    fn line(&self) -> usize {
        // current token's line number
        self.tokens[self.current].line
    }
}
