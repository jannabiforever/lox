use std::{cell::RefCell, io::Write, rc::Rc};

use super::{
    StmtAst,
    StmtParseError::{self, *},
    StmtParser,
};
use crate::{
    env::RuntimeError, error::LoxError, function::LoxFunction, literal::LoxValue, mac::tt, Env,
    Runnable,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FunctionDef<'src> {
    pub(crate) name: String,
    pub(crate) arguments: Vec<String>,
    pub(crate) body: Vec<StmtAst<'src>>,
    // end of body's bracket
    line: usize,
}

impl<'src> Runnable<'src> for FunctionDef<'src> {
    fn run<W: Write>(
        &self,
        env: Rc<RefCell<Env<'src>>>,
        _: &mut W,
    ) -> Result<Option<LoxValue<'src>>, LoxError<RuntimeError>> {
        let lox_function = LoxFunction {
            def: self.clone(),
            closure: env.clone(),
        }
        .into();
        env.borrow_mut().set(&self.name, lox_function);
        Ok(None)
    }

    fn line(&self) -> usize {
        self.line
    }
}

impl<'src> StmtParser<'src, '_> {
    pub(super) fn parse_function_def(&mut self) -> Result<FunctionDef<'src>, StmtParseError> {
        self.token_stream.next(); // Consume 'fun'.
        let name = self.expect_identifier()?;

        let mut arguments = Vec::new();
        self.expect_opening_paren()?;

        while !self.token_stream.eat(tt!(")")) {
            let argument_name = self.expect_identifier()?;
            arguments.push(argument_name);

            let peeked = self.token_stream.peek();
            if !matches!(peeked.token_type, tt!(")") | tt!(",")) {
                return Err(InvalidFunctionArgument(peeked.src.to_string()));
            } else {
                self.token_stream.eat(tt!(","));
            }
        }

        if self.token_stream.peek().token_type != tt!("{") {
            return Err(ExpectedBodyOfFunction);
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
            Err(unexpected_token) => Err(ExpectedIdent(unexpected_token.src.to_string())),
        }
    }
}
