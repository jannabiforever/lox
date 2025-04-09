use std::{cell::RefCell, io::Write, rc::Rc};

use crate::{env::Runnable, mac::tt, Env};

use super::{RuntimeError, StmtAst, StmtParseError, StmtParser};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Block {
    inner: Vec<StmtAst>,
}

impl Runnable for Block {
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<(), RuntimeError> {
        let new_env = Env::from_parent(env);
        for stmt in &self.inner {
            stmt.run(new_env.clone())?;
        }
        Ok(())
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
