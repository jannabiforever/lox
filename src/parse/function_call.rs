use std::fmt;

use crate::tokenize::tt;

use super::{ExprAst, ExprParseError, ExprParser};

#[derive(Debug, Clone)]
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
