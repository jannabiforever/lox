mod binding_power;
mod grouping;
mod literal;
mod unary;

use self::binding_power::BindingPower;

use crate::{
    error::{ErrorReporter, LoxError, ResultWithLine, WithLine},
    tokenize::{Token, TokenType},
};

use super::{expr_ast::ExprAst, ParseError};

pub(crate) struct ExprParser<'a, 'b> {
    /// Note: For reporting errors, we need to know the line number.
    tokens: &'b [WithLine<Token<'a>>],
    /// Index of the token that would be returned by [`ExprParser::next`] or [`ExprParser::peek`].
    current: usize,
}

impl<'a, 'b> ExprParser<'a, 'b> {
    pub(crate) fn new(tokens: &'b [WithLine<Token<'a>>]) -> Self {
        Self { tokens, current: 0 }
    }

    pub(crate) fn parse_with_line(&mut self) -> ResultWithLine<ExprAst, LoxError> {
        let expr = self.parse().map_err(|e| e.into());
        self.wrap(expr)
    }

    pub(crate) fn parse(&mut self) -> Result<ExprAst, ParseError> {
        self.parse_within_binding_power(BindingPower::default())
    }

    fn parse_within_binding_power(&mut self, bp: BindingPower) -> Result<ExprAst, ParseError> {
        let left = self.parse_start_of_expr_ast()?;
        loop {
            break;
        }
        Ok(left)
    }

    /// For the start of an expression, only literal, grouping, and unary are allowed.
    /// e.g. `42`, `(42)`, `!42`, `-42`
    fn parse_start_of_expr_ast(&mut self) -> Result<ExprAst, ParseError> {
        if let Some(end_node) = self.parse_end_node() {
            end_node
        } else if let Some(unary) = self.parse_unary() {
            Ok(unary?.into())
        } else {
            todo!("error handling for expected end nodes")
        }
    }

    /// End node := Literal | Grouping
    fn parse_end_node(&mut self) -> Option<Result<ExprAst, ParseError>> {
        if let Some(literal) = self.parse_literal() {
            Some(literal.map(Into::into))
        } else if let Some(grouping) = self.parse_grouping() {
            Some(grouping.map(Into::into))
        } else {
            None
        }
    }

    /// Get the next token. Panics if the end of the tokens is reached.
    /// Note: No need to return line number, because it is hanlded by [`ErrorReporter`] trait.
    fn next(&mut self) -> &Token<'a> {
        let token = self.tokens.get(self.current).unwrap().inner_ref();
        self.current += 1;
        token
    }

    /// Expect the next token to be of a certain type.
    fn expect(&mut self, token_type: TokenType) -> Result<&Token<'a>, ParseError> {
        let token = self.next();
        if token.token_type == token_type {
            Ok(token)
        } else {
            todo!("")
        }
    }

    fn eat(&mut self, allowed_token_types: &[TokenType]) -> Option<&Token<'a>> {
        let token = self.peek();
        if allowed_token_types.contains(&token.token_type) {
            Some(self.next())
        } else {
            None
        }
    }

    fn peek(&self) -> &Token<'a> {
        self.tokens.get(self.current).unwrap().inner_ref()
    }
}

impl ErrorReporter<ParseError> for ExprParser<'_, '_> {
    /// current token's line number
    fn line(&self) -> usize {
        // TODO: the correct definition of self.current is the index of the token that would be returned by [`ExprParser::next`] or [`ExprParser::peek`].
        // So, we might need to subtract 1 from self.current... but it's not that clear.
        // It is not that important for now, but it should be fixed.
        self.tokens[self.current].line
    }
}

#[cfg(test)]
mod tests {
    //! This test module is for testing single expression parsing.
    //! So it doesn't really care much about the line number.
    #![allow(unused)]
    use crate::tokenize::tt;

    use super::*;

    /// This macro only takes the vector of token types.
    fn test_expr_parse(token_types: Vec<TokenType>, expected: &str) {
        let tokens = token_types
            .iter()
            .map(|&token_type| {
                // set default src
                // TODO: maybe src should be more complete?
                let src = match token_type {
                    tt!("number") => "42",
                    _ => "sample",
                };
                WithLine::new(0, Token { token_type, src })
            })
            .collect::<Vec<_>>();

        let mut parser = ExprParser::new(&tokens);
        let expr = parser
            .parse_within_binding_power(BindingPower::default())
            .unwrap();

        assert_eq!(
            format!("{}", expr),
            expected,
            "expected: {}, got: {}",
            expected,
            expr
        );
    }

    #[test]
    fn binary_binding() {
        let _ = vec![tt!("")];
    }
}
