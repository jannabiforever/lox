use std::{cell::RefCell, fmt, io::Write, rc::Rc};

use super::ExprParser;
use crate::{
    env::{
        Env, Evaluatable,
        RuntimeError::{self, *},
    },
    error::{IntoLoxError, LoxError},
    literal::LoxValue,
    mac::tt,
    token::Token,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Variable<'a> {
    pub(crate) var: &'a Token<'a>,
}

impl fmt::Display for Variable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.var)
    }
}

impl<'a> ExprParser<'a, '_> {
    pub(super) fn try_parse_variable(&mut self) -> Option<Variable<'a>> {
        let peeked = self.token_stream.peek();
        match &peeked.token_type {
            tt!("identifier") => Some(Variable {
                var: self.token_stream.next(),
            }),
            _ => None,
        }
    }
}

impl Evaluatable for Variable<'_> {
    fn eval<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<LoxValue, LoxError<RuntimeError>> {
        if let Some(value) = env.borrow().get(self.var.src) {
            Ok(value.clone())
        } else {
            Err(UndefinedVariable(self.var.src.to_string()).at(self.line()))
        }
    }

    fn line(&self) -> usize {
        self.var.line
    }
}
