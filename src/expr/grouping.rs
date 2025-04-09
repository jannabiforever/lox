use std::{cell::RefCell, fmt, io::Write, rc::Rc};

use crate::{
    env::{Env, Evaluatable, EvaluateError},
    literal::LoxValue,
    mac::tt,
};

use super::{ExprAst, ExprParseError};

#[derive(Debug, Clone, PartialEq)]
pub struct Grouping {
    pub inner: Box<ExprAst>,
}

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(group {})", self.inner)
    }
}

impl super::ExprParser<'_, '_> {
    /// Parse a grouping expression follwing only if exists.
    /// And consume from '(' to ')'.
    pub(super) fn parse_grouping(&mut self) -> Option<Result<Grouping, ExprParseError>> {
        match self.token_stream.peek().token_type {
            tt!("(") => {
                // Consume '('.
                self.token_stream.next();

                let inner = match self.parse() {
                    Ok(inner) => Box::new(inner),
                    Err(e) => return Some(Err(e)),
                };

                if self.token_stream.expect(tt!(")")).is_ok() {
                    Some(Ok(Grouping { inner }))
                } else {
                    Some(Err(ExprParseError::ExpectedClosingParenthesis))
                }
            }
            _ => None,
        }
    }
}

impl Evaluatable for Grouping {
    fn eval<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<LoxValue, EvaluateError> {
        self.inner.eval(env)
    }
}
