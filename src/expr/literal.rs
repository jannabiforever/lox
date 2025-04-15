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
}

impl fmt::Display for LiteralExpr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token.src)
    }
}

impl Evaluatable for LiteralExpr<'_> {
    fn eval<W: Write>(&self, _: Rc<RefCell<Env<W>>>) -> Result<LoxValue, RuntimeError> {
        let value = match self.token.token_type {
            tt!("nil") => Literal::Nil,
            tt!("true") => Literal::Boolean(true),
            tt!("false") => Literal::Boolean(false),
            tt!("number") => {
                let number = self.token.src.parse::<Number>().unwrap();
                Literal::Number(number)
            }
            tt!("string") => {
                let src = self.token.src.trim_matches('"').to_string();
                Literal::String(src)
            }
            rest => unreachable!("LiteralExpr cannot be parsed from {rest:?}"),
        };

        Ok(value.into())
    }

    fn line(&self) -> usize {
        self.token.line
    }
}

impl<'a> ExprParser<'a, '_> {
    pub(super) fn try_parse_literal(&mut self) -> Option<Result<LiteralExpr<'a>, ExprParseError>> {
        let peeked = self.token_stream.peek();
        match peeked.token_type {
            tt!("nil") | tt!("true") | tt!("false") | tt!("number") | tt!("string") => {
                Some(Ok(LiteralExpr {
                    token: self.token_stream.next().clone(),
                }))
            }
            _ => None,
        }
    }
}
