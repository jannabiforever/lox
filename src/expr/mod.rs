mod assign;
mod binary;
mod binding_power;
mod error;
mod field_call;
mod function_call;
mod grouping;
mod literal;
mod unary;
mod variable;

use std::{cell::RefCell, fmt, io::Write, rc::Rc};

pub(crate) use self::{
    assign::Assign, binary::Binary, error::ExprParseError, field_call::FieldCall,
    function_call::FunctionCall, grouping::Grouping, unary::Unary, variable::Variable,
};
use self::{binding_power::BindingPower, literal::LiteralExpr};
use crate::{
    env::{Env, Evaluatable, RuntimeError},
    error::{IntoLoxError, LoxError},
    literal::LoxValue,
    mac::{impl_from, tt},
    token::TokenStream,
};

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Debug, Clone, PartialEq)]
pub enum ExprAst<'src> {
    Assign(Assign<'src>),
    Binary(Binary<'src>),
    FieldCall(FieldCall<'src>),
    FunctionCall(FunctionCall<'src>),
    Grouping(Grouping<'src>),
    LiteralExpr(LiteralExpr<'src>),
    Unary(Unary<'src>),
    Variable(Variable<'src>),
}

impl_from!(
    'src ExprAst: Assign, Binary, Grouping, FieldCall, FunctionCall, Unary, Variable, LiteralExpr
);

impl<'src> Evaluatable<'src> for ExprAst<'src> {
    fn eval<W: Write>(
        &self,
        env: Rc<RefCell<Env<'src>>>,
        stdout: &mut W,
    ) -> Result<LoxValue<'src>, LoxError<RuntimeError>> {
        match self {
            Self::Assign(v) => v.eval(env, stdout),
            Self::Binary(v) => v.eval(env, stdout),
            Self::FieldCall(_) => todo!("Implement Class"),
            Self::FunctionCall(v) => v.eval(env, stdout),
            Self::Grouping(v) => v.eval(env, stdout),
            Self::LiteralExpr(v) => v.eval(env, stdout),
            Self::Unary(v) => v.eval(env, stdout),
            Self::Variable(v) => v.eval(env, stdout),
        }
    }

    fn line(&self) -> usize {
        match self {
            Self::Assign(v) => v.line(),
            Self::Binary(v) => v.line(),
            Self::FieldCall(_) => todo!("Implement Class"),
            Self::FunctionCall(v) => v.line(),
            Self::Grouping(v) => v.line(),
            Self::LiteralExpr(v) => v.line(),
            Self::Unary(v) => v.line(),
            Self::Variable(v) => v.line(),
        }
    }
}

impl fmt::Display for ExprAst<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign(v) => write!(f, "{}", v),
            Self::Binary(v) => write!(f, "{}", v),
            Self::FieldCall(v) => write!(f, "{}", v),
            Self::FunctionCall(v) => write!(f, "{}", v),
            Self::Grouping(v) => write!(f, "{}", v),
            Self::LiteralExpr(v) => write!(f, "{}", v),
            Self::Unary(v) => write!(f, "{}", v),
            Self::Variable(v) => write!(f, "{}", v),
        }
    }
}

/// Generic 'a is for the source's lifetime.
/// Generic 'b is for the lifetime of mutable reference of token stream.
pub(crate) struct ExprParser<'src, 'mr> {
    token_stream: &'mr mut TokenStream<'src>,
}

impl<'src, 'mr> ExprParser<'src, 'mr> {
    pub(crate) fn new(token_stream: &'mr mut TokenStream<'src>) -> Self {
        Self { token_stream }
    }

    pub(crate) fn parse_with_line(&mut self) -> Result<ExprAst<'src>, LoxError<ExprParseError>> {
        self.parse().map_err(|err| err.at(self.token_stream.line()))
    }

    /// Parse within the lowest binding power.
    /// This is the entry point for parsing expressions.
    pub(crate) fn parse(&mut self) -> Result<ExprAst<'src>, ExprParseError> {
        self.parse_within_binding_power(BindingPower::default())
    }

    fn parse_within_binding_power(
        &mut self,
        bp: BindingPower,
    ) -> Result<ExprAst<'src>, ExprParseError> {
        let mut left = self.parse_start_of_expr_ast()?;
        loop {
            let token_type = self.token_stream.peek().token_type;

            // Note: this line might indicate that peeked token is ';' or ')' or '}' or eof
            // or etc... In that case, [`BindingPower::from_token_type`] returns
            // [`Bindingpower::None`], the lowest binding power,
            // so it is guaranteed that the loop will break.
            //
            // We need to break the loop and not consume the peeked token, so it can be
            // consumed by the stmt parser later.
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

    /// For the start of an expression, only literal, grouping, and unary are
    /// allowed. e.g. `42`, `(42)`, `!42`, `-42`
    fn parse_start_of_expr_ast(&mut self) -> Result<ExprAst<'src>, ExprParseError> {
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
    fn try_parse_end_node(&mut self) -> Option<Result<ExprAst<'src>, ExprParseError>> {
        if let Some(literal) = self.try_parse_literal() {
            Some(literal.map(Into::into))
        } else if let Some(variable) = self.try_parse_variable() {
            Some(Ok(variable.into()))
        } else {
            self.try_parse_grouping()
                .map(|grouping| grouping.map(Into::into))
        }
    }
}
