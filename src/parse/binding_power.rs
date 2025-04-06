use crate::tokenize::{tt, TokenType};

use super::{binary::BinaryOp, unary::UnaryOp};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord)]
pub enum BindingPower {
    /// This means that the operator cannot be binded this direction.
    #[default]
    None,
    /// a = b or c := a = (b or c), so [`BindingPower::Assign`] is lower than [`BindingPower::OrLeft`].
    Assign,
    /// a or b or c := (a or b) or c, so [`BindingPower::OrLeft`] is lower than [`BindingPower::OrRight`].
    OrLeft,
    /// a or b and c := a or (b and c), so [`BindingPower::OrRight`] is lower than [`BindingPower::AndLeft`].
    OrRight,
    /// a and b and c := (a and b) and c, so [`BindingPower::AndLeft`] is lower than [`BindingPower::AndRight`].
    AndLeft,
    /// a and b >= c := a and (b >= c), so [`BindingPower::AndRight`] is lower than [`BindingPower::Comparison`].
    AndRight,
    /// a >= b + c := a >= (b + c), so [`BindingPower::Comparison`] is lower than [`BindingPower::PlusMinusLeft`].
    Comparison,
    /// a + b - c := (a + b) - c, so [`BindingPower::PlusMinusLeft`] is lower than [`BindingPower::PlusMinusRight`].
    PlusMinusLeft,
    /// a - b * c := a - (b * c), so [`BindingPower::PlusMinusRight`] is lower than [`BindingPower::StarSlashLeft`].
    PlusMinusRight,
    /// a * b / c := (a * b) / c, so [`BindingPower::StarSlashLeft`] is lower than [`BindingPower::StarSlashRight`].
    StarSlashLeft,
    /// -a * b := (-a) * b, so [`BindingPower::StarSlashRight`] is lower than [`BindingPower::Unary`].
    StarSlashRight,
    /// -a.foo := -(a.foo), so [`BindingPower::Unary`] is lower than [`BindingPower::Call`].
    Unary,
    Call,
}

impl BindingPower {
    pub(crate) fn from_token_type(token_type: TokenType) -> (BindingPower, BindingPower) {
        // Note: - is both unary and binary operator.
        // So be careful when getting the binding power of -.
        // for most cases, it is a binary operator, because - as an unary operator,
        // is might be handled for the first expression node by [`ExprParser::try_parse_start_of_expr_ast`].
        // So check for the binary operator first.
        if let Some(op) = BinaryOp::from_token_type(token_type) {
            op.into()
        } else if let Some(op) = UnaryOp::from_token_type(token_type) {
            op.into()
        } else {
            match token_type {
                tt!("(") | tt!(".") => (Self::Call, Self::None),
                tt!("=") => (Self::Assign, Self::Assign),
                _ => (Self::None, Self::None),
            }
        }
    }
}

impl From<UnaryOp> for (BindingPower, BindingPower) {
    fn from(_: UnaryOp) -> Self {
        (BindingPower::None, BindingPower::Unary)
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
            | BinaryOp::GreaterEqual => (BindingPower::Comparison, BindingPower::Comparison),
            BinaryOp::And => (BindingPower::AndLeft, BindingPower::AndRight),
            BinaryOp::Or => (BindingPower::OrLeft, BindingPower::OrRight),
        }
    }
}
