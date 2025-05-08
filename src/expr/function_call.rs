use std::{cell::RefCell, fmt, io::Write, rc::Rc};

use super::{
    ExprAst,
    ExprParseError::{self, *},
    ExprParser,
};
use crate::{
    env::{
        Env, Evaluatable,
        RuntimeError::{self, *},
    },
    error::{IntoLoxError, LoxError},
    function::Callable,
    literal::LoxValue,
    mac::tt,
};

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall<'a> {
    pub callee: Box<ExprAst<'a>>,
    pub arguments: Vec<ExprAst<'a>>,
}

impl fmt::Display for FunctionCall<'_> {
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

impl<'a> ExprParser<'a, '_> {
    /// lhs := the function
    pub(super) fn parse_function_call(
        &mut self,
        lhs: ExprAst<'a>,
    ) -> Result<FunctionCall<'a>, ExprParseError> {
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
                            return Err(InvalidFunctionArgument(src.to_string()));
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

impl<'a> Evaluatable<'a> for FunctionCall<'a> {
    fn eval<W: Write>(
        &self,
        env: Rc<RefCell<Env<'a>>>,
        stdout: &mut W,
    ) -> Result<LoxValue<'a>, LoxError<RuntimeError>> {
        let arguments = self
            .arguments
            .iter()
            .map(|expr| expr.eval(env.clone(), stdout))
            .collect::<Result<Vec<_>, _>>()?;

        match self.callee.eval(env.clone(), stdout)? {
            LoxValue::Literal(l) => Err(InvalidCallTarget(l.to_string()).at(self.line())),
            LoxValue::RustFunction(rf) => rf
                .call(arguments, env.clone(), stdout)
                .map_err(|err| err.at(self.line())),
            LoxValue::LoxFunction(lf) => lf
                .call(arguments, env.clone(), stdout)
                .map_err(|err| err.at(self.line())),
        }
    }

    fn line(&self) -> usize {
        self.callee.line()
    }
}
