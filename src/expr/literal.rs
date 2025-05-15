use std::{cell::RefCell, fmt, io::Write, rc::Rc};

use super::ExprParser;
use crate::{
    env::RuntimeError,
    error::LoxError,
    expr::ExprParseError,
    literal::{Literal, LoxValue, Number},
    mac::tt,
    token::Token,
    Env, Evaluatable,
};

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LiteralExpr<'src> {
    pub token: Token<'src>,
}

impl LiteralExpr<'_> {
    fn eval_to_literal(&self) -> Literal {
        match self.token.token_type {
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
        }
    }
}

impl fmt::Display for LiteralExpr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.eval_to_literal())
    }
}

impl<'src> Evaluatable<'src> for LiteralExpr<'src> {
    fn eval<W: Write>(
        &self,
        _: Rc<RefCell<Env<'src>>>,
        _: &mut W,
    ) -> Result<LoxValue<'src>, LoxError<RuntimeError>> {
        let value = self.eval_to_literal();
        Ok(value.into())
    }

    fn line(&self) -> usize {
        self.token.line
    }
}

impl<'src> ExprParser<'src, '_> {
    pub(super) fn try_parse_literal(
        &mut self,
    ) -> Option<Result<LiteralExpr<'src>, ExprParseError>> {
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
