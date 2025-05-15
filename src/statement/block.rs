use std::{cell::RefCell, io::Write, rc::Rc};

use super::{RuntimeError, StmtAst, StmtParseError, StmtParser};
use crate::{env::Runnable, error::LoxError, literal::LoxValue, mac::tt, Env};

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Block<'src> {
    pub(crate) inner: Vec<StmtAst<'src>>,
    /// the line of end of brace.
    line: usize,
}

impl<'src> Runnable<'src> for Block<'src> {
    fn run<W: Write>(
        &self,
        env: Rc<RefCell<Env<'src>>>,
        stdout: &mut W,
    ) -> Result<Option<LoxValue<'src>>, LoxError<RuntimeError>> {
        let new_env = Env::from_parent(env);
        for stmt in &self.inner {
            if let Some(value) = stmt.run(new_env.clone(), stdout)? {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }

    fn line(&self) -> usize {
        self.line
    }
}

impl<'src> StmtParser<'src, '_> {
    pub(super) fn parse_block(&mut self) -> Result<Block<'src>, StmtParseError> {
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
