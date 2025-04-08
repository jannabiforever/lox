use std::{cell::RefCell, fmt, rc::Rc};

use crate::{
    env::{Env, Evaluatable, EvaluateError},
    literal::Literal,
    mac::tt,
};

use super::ExprParser;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Variable {
    pub(crate) name: String,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ExprParser<'_, '_> {
    pub(super) fn try_parse_variable(&mut self) -> Option<Variable> {
        let peeked = self.token_stream.peek();
        let src = peeked.src;
        match peeked.token_type {
            tt!("identifier") => {
                self.token_stream.next();
                Some(Variable {
                    name: src.to_string(),
                })
            }
            _ => None,
        }
    }
}

impl Evaluatable for Variable {
    fn eval(&self, env: Rc<RefCell<Env>>) -> Result<Literal, EvaluateError> {
        let name = &self.name;
        if let Some(value) = env.borrow().get(name) {
            Ok(value.clone())
        } else {
            Err(EvaluateError::UndefinedVariable(name.to_string()))
        }
    }
}
