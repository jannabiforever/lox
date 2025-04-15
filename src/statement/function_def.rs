use std::{cell::RefCell, io::Write, rc::Rc};

use super::{StmtAst, StmtParseError, StmtParser};
use crate::{
    env::RuntimeError, error::LoxError, literal::LoxValue, mac::tt, token::Token, Env, Runnable,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FunctionDef<'a> {
    name: String,
    arguments: Vec<String>,
    body: Vec<StmtAst<'a>>,
    // end of body's bracket
    line: usize,
}

impl FunctionDef<'_> {
    fn into_lox_function(&self) -> LoxValue {
        todo!("Implement function")
    }
}

impl Runnable for FunctionDef<'_> {
    fn run<W: Write>(
        &self,
        env: Rc<RefCell<Env<W>>>,
    ) -> Result<Option<LoxValue>, LoxError<RuntimeError>> {
        let lox_function = self.into_lox_function();
        env.borrow_mut().set(&self.name, lox_function);
        Ok(None)
    }

    fn line(&self) -> usize {
        self.line
    }
}

impl<'a> StmtParser<'a, '_> {
    pub(super) fn parse_function_def(&mut self) -> Result<FunctionDef<'a>, StmtParseError> {
        self.token_stream.next(); // Consume 'fun'.
        let name = self.expect_identifier()?;

        let mut arguments = Vec::new();
        self.expect_opening_paren()?;
        loop {
            match self.token_stream.peek().token_type {
                tt!(")") => break,
                _ => {
                    let argument_name = self.expect_identifier()?;
                    arguments.push(argument_name);

                    match self.token_stream.peek() {
                        Token {
                            token_type: tt!(")"),
                            ..
                        } => break,
                        Token {
                            token_type: tt!(","),
                            ..
                        } => {
                            self.token_stream.next();
                            continue;
                        }
                        rest => {
                            return Err(StmtParseError::InvalidFunctionArgument(
                                rest.src.to_string(),
                            ))
                        }
                    }
                }
            }
        }
        self.expect_closing_paren()?;
        if self.token_stream.peek().token_type != tt!("{") {
            return Err(StmtParseError::ExpectedBodyOfFunction);
        }

        let (line, body) = {
            let block = self.parse_block()?;
            (block.line(), block.inner)
        };

        Ok(FunctionDef {
            name,
            arguments,
            body,
            line,
        })
    }

    fn expect_identifier(&mut self) -> Result<String, StmtParseError> {
        match self.token_stream.expect(tt!("identifier")) {
            Ok(token) => Ok(token.src.to_string()),
            Err(unexpected_token) => Err(StmtParseError::ExpectedIdent(
                unexpected_token.src.to_string(),
            )),
        }
    }
}
