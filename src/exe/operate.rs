use std::ops::Deref;

use crate::{
    error::RuntimeError::{self, *},
    expr_ast::{BinaryOp, UnaryOp},
    literal::Literal,
};

pub fn unary_function(op: UnaryOp) -> fn(Literal) -> Result<Literal, RuntimeError> {
    match op {
        UnaryOp::Minus => |v| match v {
            Literal::Number(n) => Ok(Literal::Number(-n)),
            _ => Err(UnaryOperandType("number")),
        },
        UnaryOp::Bang => |v| Ok(Literal::Boolean(!v.is_truthy())),
    }
}

pub fn binary_function<T: Sized + Deref<Target = Result<Literal, RuntimeError>>>(
    op: BinaryOp,
) -> fn(Literal, T) -> Result<Literal, RuntimeError> {
    match op {
        BinaryOp::Plus => |left, right| match (left, (*right).clone()?) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
            (Literal::String(l), Literal::String(r)) => Ok(Literal::String(l + &r)),
            _ => Err(BinaryOperandType("two numbers or two strings")),
        },

        BinaryOp::Minus => |left, right| match (left, (*right).clone()) {
            (Literal::Number(l), Ok(Literal::Number(r))) => Ok(Literal::Number(l - r)),
            _ => Err(BinaryOperandType("two numbers")),
        },

        BinaryOp::Star => |left, right| match (left, (*right).clone()?) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l * r)),
            _ => Err(BinaryOperandType("two numbers")),
        },

        BinaryOp::Slash => |left, right| match (left, (*right).clone()?) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l / r)),
            _ => Err(BinaryOperandType("two numbers")),
        },

        BinaryOp::EqualEqual => |left, right| Ok(Literal::Boolean(left == (*right).clone()?)),

        BinaryOp::BangEqual => |left, right| Ok(Literal::Boolean(left != (*right).clone()?)),

        BinaryOp::Greater => |left, right| match (left, (*right).clone()?) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Boolean(l > r)),
            _ => Err(BinaryOperandType("two numbers")),
        },

        BinaryOp::GreaterEqual => |left, right| match (left, (*right).clone()?) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Boolean(l >= r)),
            _ => Err(BinaryOperandType("two numbers")),
        },

        BinaryOp::Less => |left, right| match (left, (*right).clone()?) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Boolean(l < r)),
            _ => Err(BinaryOperandType("two numbers")),
        },

        BinaryOp::LessEqual => |left, right| match (left, (*right).clone()?) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Boolean(l <= r)),
            _ => Err(BinaryOperandType("two numbers")),
        },

        BinaryOp::And => |left, right| {
            let res = if left.is_truthy() {
                // evaluate the right side only if the left side is truthy
                (*right).clone()?.is_truthy()
            } else {
                false
            };

            Ok(Literal::Boolean(res))
        },
        BinaryOp::Or => |left, right| {
            let res = if left.is_truthy() {
                true
            } else {
                // evaluate the right side only if the left side is not truthy
                (*right).clone()?.is_truthy()
            };

            Ok(Literal::Boolean(res))
        },
    }
}
