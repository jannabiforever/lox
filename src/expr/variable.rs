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

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Variable<'src> {
    pub(crate) var: &'src Token<'src>,
}

impl fmt::Display for Variable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.var)
    }
}

impl<'src> ExprParser<'src, '_> {
    pub(super) fn try_parse_variable(&mut self) -> Option<Variable<'src>> {
        let peeked = self.token_stream.peek();
        match &peeked.token_type {
            tt!("identifier") => Some(Variable {
                var: self.token_stream.next(),
            }),
            _ => None,
        }
    }
}

impl<'src> Evaluatable<'src> for Variable<'src> {
    fn eval<W: Write>(
        &self,
        env: Rc<RefCell<Env<'src>>>,
        _: &mut W,
    ) -> Result<LoxValue<'src>, LoxError<RuntimeError>> {
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
