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

pub fn binary_function(op: BinaryOp) -> fn(Literal, Literal) -> Result<Literal, RuntimeError> {
    match op {
        BinaryOp::Plus => |left, right| match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
            (Literal::String(l), Literal::String(r)) => Ok(Literal::String(l + &r)),
            _ => Err(BinaryOperandType("two numbers or two strings")),
        },
        BinaryOp::Minus => |left, right| match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l - r)),
            _ => Err(BinaryOperandType("two numbers")),
        },

        BinaryOp::Star => |left, right| match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l * r)),
            _ => Err(BinaryOperandType("two numbers")),
        },

        BinaryOp::Slash => |left, right| match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l / r)),
            _ => Err(BinaryOperandType("two numbers")),
        },

        BinaryOp::EqualEqual => |left, right| Ok(Literal::Boolean(left == right)),

        BinaryOp::BangEqual => |left, right| Ok(Literal::Boolean(left != right)),

        BinaryOp::Greater => |left, right| match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Boolean(l > r)),
            _ => Err(BinaryOperandType("two numbers")),
        },

        BinaryOp::GreaterEqual => |left, right| match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Boolean(l >= r)),
            _ => Err(BinaryOperandType("two numbers")),
        },

        BinaryOp::Less => |left, right| match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Boolean(l < r)),
            _ => Err(BinaryOperandType("two numbers")),
        },

        BinaryOp::LessEqual => |left, right| match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Boolean(l <= r)),
            _ => Err(BinaryOperandType("two numbers")),
        },

        BinaryOp::And => todo!("Logical AND"),
        BinaryOp::Or => todo!("Logical OR"),
    }
}
