mod assign;
mod binary;
mod binding_power;
mod error;
mod field_call;
mod function_call;
mod grouping;
mod literal;
mod unary;

use std::fmt;

pub(crate) use self::assign::Assign;
pub(crate) use self::binary::{Binary, BinaryOp};
pub(crate) use self::error::ExprParseError;
pub(crate) use self::field_call::FieldCall;
pub(crate) use self::function_call::FunctionCall;
pub(crate) use self::grouping::Grouping;
pub(crate) use self::unary::{Unary, UnaryOp};

use self::binding_power::BindingPower;

use crate::mac::impl_from;
use crate::{
    error::IntoLoxError,
    literal::Literal,
    tokenize::{tt, TokenStream},
    LoxError,
};

#[derive(Debug, Clone)]
pub enum ExprAst {
    Assign(Assign),
    Binary(Binary),
    FieldCall(FieldCall),
    FunctionCall(FunctionCall),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Variable(String),
}

impl_from!(
    ExprAst: Assign, Binary, Grouping, FieldCall, FunctionCall, Literal, Unary
);

impl fmt::Display for ExprAst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign(v) => write!(f, "{}", v),
            Self::Binary(v) => write!(f, "{}", v),
            Self::FieldCall(v) => write!(f, "{}", v),
            Self::FunctionCall(v) => write!(f, "{}", v),
            Self::Grouping(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
            Self::Unary(v) => write!(f, "{}", v),
            Self::Variable(v) => write!(f, "{}", v),
        }
    }
}

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
    pub(crate) fn parse(&mut self) -> Result<ExprAst, ExprParseError> {
        self.parse_within_binding_power(BindingPower::default())
    }

    fn parse_within_binding_power(&mut self, bp: BindingPower) -> Result<ExprAst, ExprParseError> {
        let mut left = self.parse_start_of_expr_ast()?;
        loop {
            let token_type = self.token_stream.peek().token_type;

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
    fn parse_start_of_expr_ast(&mut self) -> Result<ExprAst, ExprParseError> {
        if let Some(end_node) = self.try_parse_end_node() {
            end_node
        } else if let Some(unary) = self.try_parse_unary() {
            Ok(unary?.into())
        } else {
            let token = self.token_stream.next();
            Err(ExprParseError::ExpectedExpression(token.src.to_string()))
        }
    }

    /// End node := Literal | Grouping
    fn try_parse_end_node(&mut self) -> Option<Result<ExprAst, ExprParseError>> {
        if let Some(literal) = self.parse_literal() {
            Some(literal.map(Into::into))
        } else if let Some(variable) = self.try_parse_variable() {
            Some(Ok(ExprAst::Variable(variable)))
        } else {
            self.parse_grouping()
                .map(|grouping| grouping.map(Into::into))
        }
    }

    fn line(&self) -> usize {
        self.token_stream.line()
    }
}
