mod binary;
mod binding_power;
mod grouping;
mod literal;
mod unary;

use self::binding_power::BindingPower;

use crate::{
    error::{ErrorReporter, LoxError, ResultWithLine, WithLine},
    tokenize::{tt, Token, TokenType},
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
        let mut left = self.try_parse_start_of_expr_ast()?;
        loop {
            let token_type = self.peek().token_type;

            // Note: this line might indicate that peeked token is ';' or ')' or '}' or eof or etc...
            // In that case, [`BindingPower::from_token_type`] returns [`Bindingpower::None`], the lowest binding power,
            // so it is guaranteed that the loop will break.
            //
            // We need to break the loop and not consume the peeked token, so it can be consumed by the stmt parser later.
            if dbg!(BindingPower::from_token_type(token_type)).0 <= bp {
                break;
            }

            match token_type {
                tt!("=") => {
                    todo!("assignment")
                }
                tt!(".") => {
                    todo!("field call")
                }
                tt!("(") => {
                    todo!("function call")
                }
                _ => {
                    if let Some(binary) = self.try_parse_binary(left.clone()) {
                        left = binary?.into();
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(left)
    }

    /// For the start of an expression, only literal, grouping, and unary are allowed.
    /// e.g. `42`, `(42)`, `!42`, `-42`
    fn try_parse_start_of_expr_ast(&mut self) -> Result<ExprAst, ParseError> {
        if let Some(end_node) = self.try_parse_end_node() {
            end_node
        } else if let Some(unary) = self.parse_unary() {
            Ok(unary?.into())
        } else {
            todo!("error handling for expected end nodes")
        }
    }

    /// End node := Literal | Grouping
    fn try_parse_end_node(&mut self) -> Option<Result<ExprAst, ParseError>> {
        if let Some(literal) = self.parse_literal() {
            Some(literal.map(Into::into))
        } else {
            self.parse_grouping()
                .map(|grouping| grouping.map(Into::into))
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
