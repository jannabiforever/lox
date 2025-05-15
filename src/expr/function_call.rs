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
pub struct FunctionCall<'src> {
    pub callee: Box<ExprAst<'src>>,
    pub arguments: Vec<ExprAst<'src>>,
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

impl<'src> ExprParser<'src, '_> {
    /// lhs := the function
    pub(super) fn parse_function_call(
        &mut self,
        lhs: ExprAst<'src>,
    ) -> Result<FunctionCall<'src>, ExprParseError> {
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

impl<'src> Evaluatable<'src> for FunctionCall<'src> {
    fn eval<W: Write>(
        &self,
        env: Rc<RefCell<Env<'src>>>,
        stdout: &mut W,
    ) -> Result<LoxValue<'src>, LoxError<RuntimeError>> {
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
