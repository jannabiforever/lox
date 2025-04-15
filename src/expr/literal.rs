use std::{cell::RefCell, fmt, io::Write, rc::Rc};

use super::ExprParser;
use crate::{
    env::RuntimeError,
    expr::ExprParseError,
    literal::{Literal, LoxValue, Number},
    mac::tt,
    token::Token,
    Env, Evaluatable,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LiteralExpr<'a> {
    pub token: Token<'a>,
    pub literal: Literal,
}

impl fmt::Display for LiteralExpr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token.src)
    }
}

impl Evaluatable for LiteralExpr<'_> {
    fn eval<W: Write>(&self, _: Rc<RefCell<Env<W>>>) -> Result<LoxValue, RuntimeError> {
        Ok(self.literal.clone().into())
    }
}

impl<'a> ExprParser<'a, '_> {
    pub(super) fn try_parse_literal(&mut self) -> Option<Result<LiteralExpr<'a>, ExprParseError>> {
        let peeked = self.token_stream.peek();
        let (token, literal) = match peeked.token_type {
            tt!("nil") => (self.token_stream.next().clone(), Literal::Nil),
            tt!("true") => (self.token_stream.next().clone(), Literal::Boolean(true)),
            tt!("false") => (self.token_stream.next().clone(), Literal::Boolean(false)),
            tt!("number") => {
                let num = peeked.src.parse::<Number>().unwrap();
                (self.token_stream.next().clone(), Literal::Number(num))
            }
            tt!("string") => {
                let src = peeked.src.trim_matches('"').to_string();
                (self.token_stream.next().clone(), Literal::String(src))
            }
            _ => return None,
        };

        Some(Ok(LiteralExpr { token, literal }))
    }
}
