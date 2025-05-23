use std::{cell::RefCell, fmt, io::Write, rc::Rc};

use super::{binding_power::BindingPower, ExprAst, ExprParseError, ExprParser};
use crate::{
    env::{
        Env, Evaluatable,
        RuntimeError::{self, *},
    },
    error::{IntoLoxError, LoxError},
    literal::{Literal, LoxValue},
    mac::tt,
    token::TokenType,
};

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Debug, Clone, PartialEq)]
pub struct Unary<'src> {
    pub op: UnaryOp,
    pub right: Box<ExprAst<'src>>,
}

impl fmt::Display for Unary<'_> {
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

impl<'src> ExprParser<'src, '_> {
    /// Parse a unary expression following only if exists.
    /// And consume from unary operator(!, -) to the right operand.
    pub(super) fn try_parse_unary(&mut self) -> Option<Result<Unary<'src>, ExprParseError>> {
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

impl<'src> Evaluatable<'src> for Unary<'src> {
    fn eval<W: Write>(
        &self,
        env: Rc<RefCell<Env<'src>>>,
        stdout: &mut W,
    ) -> Result<LoxValue<'src>, LoxError<RuntimeError>> {
        let right = self.right.eval(env.clone(), stdout)?;

        match self.op {
            UnaryOp::Minus => {
                if let LoxValue::Literal(Literal::Number(num)) = right {
                    Ok(Literal::Number(-num).into())
                } else {
                    Err(OperandMustBe("number").at(self.line()))
                }
            }
            UnaryOp::Bang => {
                let literal: Literal = (!right.is_literal_and(|l| l.is_truthy())).into();
                Ok(literal.into())
            }
        }
    }

    fn line(&self) -> usize {
        self.right.line()
    }
}
