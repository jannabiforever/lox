use std::{cell::RefCell, io::Write, rc::Rc};

use super::{RuntimeError, StmtAst, StmtParseError, StmtParser};
use crate::{env::Runnable, literal::LoxValue, mac::tt, Env};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Block<'a> {
    pub(crate) inner: Vec<StmtAst<'a>>,
    /// the line of end of brace.
    line: usize,
}

impl Runnable for Block<'_> {
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<Option<LoxValue>, RuntimeError> {
        let new_env = Env::from_parent(env);
        for stmt in &self.inner {
            if let Some(value) = stmt.run(new_env.clone())? {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }

    fn line(&self) -> usize {
        self.line
    }
}

impl<'a> StmtParser<'a, '_> {
    pub(super) fn parse_block(&mut self) -> Result<Block<'a>, StmtParseError> {
        let mut inner = Vec::new();

        self.token_stream.next(); // Consume '{'.
        while self.token_stream.peek().token_type != tt!("}") {
            let next_stmt = self.parse()?;
            inner.push(next_stmt);
        }

        match self.token_stream.expect(tt!("}")) {
            Ok(end_brace) => Ok(Block {
                inner,
                line: end_brace.line,
            }),
            Err(unexpected_token) => Err(StmtParseError::ExpectedEndOfBracket(
                unexpected_token.src.to_string(),
            )),
        }
    }
}
