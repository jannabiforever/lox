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

use std::cell::RefCell;
use std::fmt;
use std::io::Write;
use std::rc::Rc;

pub(crate) use self::assign::Assign;
pub(crate) use self::binary::Binary;
use self::binding_power::BindingPower;
pub(crate) use self::error::ExprParseError;
pub(crate) use self::field_call::FieldCall;
pub(crate) use self::function_call::FunctionCall;
pub(crate) use self::grouping::Grouping;
pub(crate) use self::unary::Unary;
pub(crate) use self::variable::Variable;
use crate::env::Env;
use crate::env::Evaluatable;
use crate::env::RuntimeError;
use crate::error::IntoLoxError;
use crate::error::LoxError;
use crate::literal::Literal;
use crate::literal::LoxValue;
use crate::mac::impl_from;
use crate::mac::tt;
use crate::token::TokenStream;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprAst {
    Assign(Assign),
    Binary(Binary),
    FieldCall(FieldCall),
    FunctionCall(FunctionCall),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
}

impl_from!(
    ExprAst: Assign, Binary, Grouping, FieldCall, FunctionCall, Literal, Unary, Variable
);

impl Evaluatable for ExprAst {
    fn eval<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<LoxValue, RuntimeError> {
        match self {
            Self::Assign(v) => v.eval(env),
            Self::Binary(v) => v.eval(env),
            Self::FieldCall(_) => todo!(),
            Self::FunctionCall(v) => v.eval(env),
            Self::Grouping(v) => v.eval(env),
            Self::Literal(v) => v.eval(env),
            Self::Unary(v) => v.eval(env),
            Self::Variable(v) => v.eval(env),
        }
    }
}

impl Evaluatable for Option<ExprAst> {
    fn eval<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<LoxValue, RuntimeError> {
        self.as_ref()
            .map(|expr| expr.eval(env))
            .transpose()
            .map(|s| s.unwrap_or_default())
    }
}

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

    pub(crate) fn parse_with_line(&mut self) -> Result<ExprAst, LoxError<ExprParseError>> {
        self.parse().map_err(|e| e.error(self.token_stream.line()))
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
            Some(Ok(variable.into()))
        } else {
            self.parse_grouping()
                .map(|grouping| grouping.map(Into::into))
        }
    }
}
