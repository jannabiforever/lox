use std::cell::RefCell;
use std::fmt;
use std::io::Write;
use std::rc::Rc;

use super::ExprAst;
use super::ExprParseError;
use super::ExprParser;
use crate::env::Env;
use crate::env::Evaluatable;
use crate::env::RuntimeError;
use crate::function::Callable;
use crate::literal::LoxValue;
use crate::mac::tt;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub callee: Box<ExprAst>,
    pub arguments: Vec<ExprAst>,
}

impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.callee)?;
        for (i, arg) in self.arguments.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, ")")
    }
}

impl ExprParser<'_, '_> {
    /// lhs := the function
    pub(super) fn parse_function_call(
        &mut self,
        lhs: ExprAst,
    ) -> Result<FunctionCall, ExprParseError> {
        self.token_stream.next(); // consume the '('
        let mut arguments = Vec::new();

        loop {
            match self.token_stream.peek().token_type {
                tt!(")") => {
                    self.token_stream.next(); // consume the ')'
                    break;
                }
                _ => {
                    let argument = self.parse()?;
                    arguments.push(argument);

                    let peeked = self.token_stream.peek();
                    let src = peeked.src;
                    match peeked.token_type {
                        tt!(")") => {
                            self.token_stream.next();
                            break;
                        }
                        tt!(",") => {
                            self.token_stream.next();
                            continue;
                        }
                        _ => {
                            return Err(ExprParseError::InvalidFunctionArgument(src.to_string()));
                        }
                    }
                }
            }
        }

        Ok(FunctionCall {
            callee: Box::new(lhs),
            arguments,
        })
    }
}

impl Evaluatable for FunctionCall {
    fn eval<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<LoxValue, RuntimeError> {
        let arguments = self
            .arguments
            .iter()
            .map(|expr| expr.eval(env.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        match self.callee.eval(env.clone())? {
            LoxValue::Literal(l) => Err(RuntimeError::InvalidCallTarget(l.to_string())),
            LoxValue::RustFunction(rf) => rf.call(arguments, env.clone()),
            LoxValue::LoxFunction(lf) => lf.call(arguments, env.clone()),
        }
    }
}
