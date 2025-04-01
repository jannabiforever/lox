use crate::parse::{
    expr_ast::{Unary, UnaryOp},
    ExprParseError,
};

use super::binding_power::BindingPower;

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
