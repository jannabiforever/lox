use std::cell::RefCell;
use std::fmt;
use std::io::Write;
use std::rc::Rc;

use super::binding_power::BindingPower;
use super::ExprAst;
use super::ExprParseError;
use crate::env::Env;
use crate::env::Evaluatable;
use crate::env::RuntimeError;
use crate::literal::Literal;
use crate::literal::LoxValue;
use crate::mac::tt;
use crate::token::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Unary {
    pub op: UnaryOp,
    pub right: Box<ExprAst>,
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.op, self.right)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Bang,
    Minus,
}

impl UnaryOp {
    pub(crate) fn from_token_type(token_type: TokenType) -> Option<UnaryOp> {
        match token_type {
            tt!("!") => Some(Self::Bang),
            tt!("-") => Some(Self::Minus),
            _ => None,
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bang => write!(f, "!"),
            Self::Minus => write!(f, "-"),
        }
    }
}

impl super::ExprParser<'_, '_> {
    /// Parse a unary expression following only if exists.
    /// And consume from unary operator(!, -) to the right operand.
    pub(super) fn try_parse_unary(&mut self) -> Option<Result<Unary, ExprParseError>> {
        let op = self.eat_unary_op()?;

        let right = match self.parse_within_binding_power(BindingPower::Unary) {
            Ok(inner) => Box::new(inner),
            Err(e) => return Some(Err(e)),
        };

        Some(Ok(Unary { op, right }))
    }

    /// Consume a unary operator if exists.
    fn eat_unary_op(&mut self) -> Option<UnaryOp> {
        let token_type = self.token_stream.peek().token_type;
        UnaryOp::from_token_type(token_type).inspect(|_| {
            self.token_stream.next();
        })
    }
}

impl Evaluatable for Unary {
    fn eval<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<LoxValue, RuntimeError> {
        let right = self.right.eval(env.clone())?;

        match self.op {
            UnaryOp::Minus => {
                if let LoxValue::Literal(Literal::Number(num)) = right {
                    Ok(Literal::Number(-num).into())
                } else {
                    Err(RuntimeError::OperandMustBe("number"))
                }
            }
            UnaryOp::Bang => {
                let literal: Literal = (!right.is_literal_and(|l| l.is_truthy())).into();
                Ok(literal.into())
            }
        }
    }
}
