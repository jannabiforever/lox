use crate::parse::{
    expr_ast::{Binary, BinaryOp, ExprAst},
    ParseError,
};

use super::binding_power::BindingPower;

impl super::ExprParser<'_, '_> {
    /// If following token is a binary operator,
    /// parse the right operand and return the binary expression,
    /// consuming the operator and the right operand.
    ///
    /// Otherwise, it doesn't consume anything and returns `None`.
    pub(super) fn try_parse_binary(&mut self, lhs: ExprAst) -> Option<Result<Binary, ParseError>> {
        let op = self.eat_binary_op()?;

        let binding_power: (BindingPower, BindingPower) = op.into();
        let right_binding_power = binding_power.1;

        let right = match self.parse_within_binding_power(right_binding_power) {
            Ok(inner) => Box::new(inner),
            Err(e) => return Some(Err(e)),
        };

        Some(Ok(Binary {
            left: Box::new(lhs),
            op,
            right,
        }))
    }

    fn eat_binary_op(&mut self) -> Option<BinaryOp> {
        let token_type = self.peek().token_type;
        BinaryOp::from_token_type(token_type).inspect(|_| {
            self.next();
        })
    }
}
