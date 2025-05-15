use std::{cell::RefCell, fmt, io::Write, rc::Rc};

use super::{
    ExprAst,
    ExprParseError::{self, *},
    ExprParser,
};
use crate::{
    env::{Env, Evaluatable, RuntimeError},
    error::LoxError,
    literal::LoxValue,
    mac::tt,
};

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Debug, Clone, PartialEq)]
pub struct Grouping<'src> {
    pub inner: Box<ExprAst<'src>>,
}

impl fmt::Display for Grouping<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(group {})", self.inner)
    }
}

impl<'src> ExprParser<'src, '_> {
    /// Parse a grouping expression follwing only if exists.
    /// And consume from '(' to ')'.
    pub(super) fn try_parse_grouping(&mut self) -> Option<Result<Grouping<'src>, ExprParseError>> {
        match self.token_stream.peek().token_type {
            tt!("(") => {
                self.token_stream.next(); // Consume '('.

                let inner = match self.parse() {
                    Ok(inner) => Box::new(inner),
                    Err(e) => return Some(Err(e)),
                };

                if self.token_stream.expect(tt!(")")).is_ok() {
                    Some(Ok(Grouping { inner }))
                } else {
                    Some(Err(ExpectedClosingParenthesis))
                }
            }
            _ => None,
        }
    }
}

impl<'src> Evaluatable<'src> for Grouping<'src> {
    fn eval<W: Write>(
        &self,
        env: Rc<RefCell<Env<'src>>>,
        stdout: &mut W,
    ) -> Result<LoxValue<'src>, LoxError<RuntimeError>> {
        self.inner.eval(env, stdout)
    }

    fn line(&self) -> usize {
        self.inner.line()
    }
}
