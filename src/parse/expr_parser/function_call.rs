use crate::{
    parse::{
        expr_ast::{ExprAst, FunctionCall},
        ParseError,
    },
    tokenize::tt,
};

use super::ExprParser;

impl ExprParser<'_, '_> {
    /// lhs := the function
    pub(super) fn parse_function_call(&mut self, lhs: ExprAst) -> Result<FunctionCall, ParseError> {
        self.next(); // consume the '('
        let mut arguments = Vec::new();

        loop {
            match self.peek().token_type {
                tt!(")") => {
                    self.next(); // consume the ')'
                    break;
                }
                _ => {
                    let argument = self.parse()?;
                    arguments.push(argument);

                    let peeked = self.peek();
                    let src = peeked.src;
                    match peeked.token_type {
                        tt!(")") => {
                            self.next();
                            break;
                        }
                        tt!(",") => {
                            self.next();
                            continue;
                        }
                        _ => {
                            return Err(ParseError::InvalidFunctionArgument(src.to_string()));
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
