use crate::{
    parse::{
        expr_ast::{Operator, Unary, UnaryOp},
        ParseError,
    },
    tokenize::tt,
};

impl super::ExprParser<'_, '_> {
    pub(super) fn parse_unary(&mut self) -> Option<Result<Unary, ParseError>> {
        let peeked = self.peek();
        let op = match peeked.token_type {
            tt!("!") => {
                self.next();
                UnaryOp::Bang
            }

            tt!("-") => {
                self.next();
                UnaryOp::Minus
            }

            _ => return None,
        };

        let right = match self.parse_within_binding_power(op.binding_power()) {
            Ok(inner) => Box::new(inner),
            Err(e) => return Some(Err(e)),
        };

        Some(Ok(Unary { op, right }))
    }
}
