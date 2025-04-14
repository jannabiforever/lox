use std::cell::RefCell;
use std::fmt;
use std::io::Write;
use std::rc::Rc;

use super::ExprParser;
use crate::env::Env;
use crate::env::Evaluatable;
use crate::env::RuntimeError;
use crate::literal::LoxValue;
use crate::mac::tt;

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
    fn eval<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<LoxValue, RuntimeError> {
        let name = &self.name;
        if let Some(value) = env.borrow().get(name) {
            Ok(value.clone())
        } else {
            Err(RuntimeError::UndefinedVariable(name.to_string()))
        }
    }
}
