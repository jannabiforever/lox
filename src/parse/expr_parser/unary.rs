use crate::{
    parse::{
        expr_ast::{Unary, UnaryOp},
        ParseError,
    },
    tokenize::tt,
};

use super::binding_power::BindingPower;

impl super::ExprParser<'_, '_> {
    /// Parse a unary expression following only if exists.
    /// And consume from unary operator(!, -) to the right operand.
    pub(super) fn parse_unary(&mut self) -> Option<Result<Unary, ParseError>> {
        let op = match self.eat(&[tt!("!"), tt!("-")])?.token_type {
            tt!("!") => UnaryOp::Bang,
            tt!("-") => UnaryOp::Minus,
            _ => unreachable!("Already handled by `eat`"),
        };

        let right = match self.parse_within_binding_power(BindingPower::Unary) {
            Ok(inner) => Box::new(inner),
            Err(e) => return Some(Err(e)),
        };

        Some(Ok(Unary { op, right }))
    }
}
