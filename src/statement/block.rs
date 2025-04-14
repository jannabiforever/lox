use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use super::RuntimeError;
use super::StmtAst;
use super::StmtParseError;
use super::StmtParser;
use crate::env::Runnable;
use crate::literal::LoxValue;
use crate::mac::tt;
use crate::Env;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Block {
    pub(crate) inner: Vec<StmtAst>,
}

impl Runnable for Block {
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<Option<LoxValue>, RuntimeError> {
        let new_env = Env::from_parent(env);
        for stmt in &self.inner {
            if let Some(value) = stmt.run(new_env.clone())? {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }
}

impl StmtParser<'_, '_> {
    pub(super) fn parse_block(&mut self) -> Result<Block, StmtParseError> {
        let mut inner = Vec::new();

        self.token_stream.next(); // Consume '{'.
        while self.token_stream.peek().token_type != tt!("}") {
            let next_stmt = self.parse()?;
            inner.push(next_stmt);
        }

        self.token_stream
            .expect(tt!("}"))
            .map_err(|unexpected_token| {
                StmtParseError::ExpectedEndOfBracket(unexpected_token.src.to_string())
            })?;

        Ok(Block { inner })
    }
}
