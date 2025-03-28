use crate::{
    parse::expr_ast::{BinaryOp, UnaryOp},
    tokenize::{tt, TokenType},
};

static LEFT_ASSOCIATIVE_OPERATORS: &[TokenType] = &[
    // Assignment
    tt!("="),
    // Binary operators
    tt!("+"),
    tt!("-"),
    tt!("*"),
    tt!("/"),
    tt!("=="),
    tt!("!="),
    tt!("<"),
    tt!("<="),
    tt!(">"),
    tt!(">="),
    tt!("and"),
    tt!("or"),
    // Field call
    tt!("."),
    // Function call
    tt!("("),
];

static RIGHT_ASSOCIATIVE_OPERATORS: &[TokenType] = &[
    // Assignment
    tt!("="),
    // Binary operators
    tt!("+"),
    tt!("-"),
    tt!("*"),
    tt!("/"),
    tt!("=="),
    tt!("!="),
    tt!("<"),
    tt!("<="),
    tt!(">"),
    tt!(">="),
    tt!("and"),
    tt!("or"),
    // Unary
    tt!("!"),
    tt!("-"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BindingPower {
    #[default]
    /// This means that the operator cannot be binded this direction.
    None,
    PlusMinusLeft,
    PlusMinusRight,
    StarSlashLeft,
    StarSlashRight,
    ComparisonLeft,
    ComparisonRight,
    AndLeft,
    AndRight,
    OrLeft,
    OrRight,
    Unary,
    Call,
}

impl From<UnaryOp> for (BindingPower, BindingPower) {
    fn from(op: UnaryOp) -> Self {
        match op {
            UnaryOp::Bang | UnaryOp::Minus => (BindingPower::None, BindingPower::Unary),
        }
    }
}

impl From<BinaryOp> for (BindingPower, BindingPower) {
    fn from(op: BinaryOp) -> Self {
        match op {
            BinaryOp::Plus | BinaryOp::Minus => {
                (BindingPower::PlusMinusLeft, BindingPower::PlusMinusRight)
            }
            BinaryOp::Star | BinaryOp::Slash => {
                (BindingPower::StarSlashLeft, BindingPower::StarSlashRight)
            }
            BinaryOp::EqualEqual
            | BinaryOp::BangEqual
            | BinaryOp::Less
            | BinaryOp::LessEqual
            | BinaryOp::Greater
            | BinaryOp::GreaterEqual => {
                (BindingPower::ComparisonLeft, BindingPower::ComparisonRight)
            }
            BinaryOp::And => (BindingPower::AndLeft, BindingPower::AndRight),
            BinaryOp::Or => (BindingPower::OrLeft, BindingPower::OrRight),
        }
    }
}
