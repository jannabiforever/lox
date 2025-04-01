mod assign;
mod binary;
mod binding_power;
mod field_call;
mod function_call;
mod grouping;
mod literal;
mod unary;

use self::binding_power::BindingPower;

use crate::{
    error::IntoLoxError,
    tokenize::{tt, Token, TokenStream, TokenType},
    LoxError,
};

use super::{expr_ast::ExprAst, ParseError};

pub(crate) struct ExprParser<'a, 'b> {
    token_stream: &'b mut TokenStream<'a>,
}

impl<'a, 'b> ExprParser<'a, 'b> {
    pub(crate) fn new(token_stream: &'b mut TokenStream<'a>) -> Self {
        Self { token_stream }
    }

    pub(crate) fn parse_with_line(&mut self) -> Result<ExprAst, LoxError> {
        self.parse().map_err(|e| e.error(self.line()))
    }

    /// Parse within the lowest binding power.
    /// This is the entry point for parsing expressions.
    pub(crate) fn parse(&mut self) -> Result<ExprAst, ParseError> {
        self.parse_within_binding_power(BindingPower::default())
    }

    fn parse_within_binding_power(&mut self, bp: BindingPower) -> Result<ExprAst, ParseError> {
        let mut left = self.parse_start_of_expr_ast()?;
        loop {
            let token_type = self.peek().token_type;

            // Note: this line might indicate that peeked token is ';' or ')' or '}' or eof or etc...
            // In that case, [`BindingPower::from_token_type`] returns [`Bindingpower::None`], the lowest binding power,
            // so it is guaranteed that the loop will break.
            //
            // We need to break the loop and not consume the peeked token, so it can be consumed by the stmt parser later.
            if BindingPower::from_token_type(token_type).0 <= bp {
                break;
            }

            match token_type {
                tt!("=") => {
                    left = self.parse_assign(left)?.into();
                }
                tt!(".") => {
                    left = self.parse_field_call(left)?.into();
                }
                tt!("(") => {
                    left = self.parse_function_call(left)?.into();
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
    fn parse_start_of_expr_ast(&mut self) -> Result<ExprAst, ParseError> {
        if let Some(end_node) = self.try_parse_end_node() {
            end_node
        } else if let Some(unary) = self.try_parse_unary() {
            Ok(unary?.into())
        } else {
            let token = self.next();
            Err(ParseError::ExpectedExpression(token.src.to_string()))
        }
    }

    /// End node := Literal | Grouping
    fn try_parse_end_node(&mut self) -> Option<Result<ExprAst, ParseError>> {
        if let Some(literal) = self.parse_literal() {
            Some(literal.map(Into::into))
        } else if let Some(variable) = self.try_parse_variable() {
            Some(Ok(ExprAst::Variable(variable)))
        } else {
            self.parse_grouping()
                .map(|grouping| grouping.map(Into::into))
        }
    }

    /// Get the next token. Panics if the end of the tokens is reached.
    /// Note: No need to return line number, because it is hanlded by [`ErrorReporter`] trait.
    fn next(&mut self) -> &'a Token<'a> {
        self.token_stream.next()
    }

    /// Expect the next token to be of a certain type.
    /// If it is, return Ok(token) else return Err(token).
    fn expect(&mut self, token_type: TokenType) -> Result<&Token<'a>, &Token<'a>> {
        let token = self.next();
        if token.token_type == token_type {
            Ok(token)
        } else {
            Err(token)
        }
    }

    fn peek(&self) -> &'a Token<'a> {
        self.token_stream.peek()
    }

    fn line(&self) -> usize {
        self.token_stream.line()
    }
}
