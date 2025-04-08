use crate::tokenize::tt;

use super::{Runtime, RuntimeError, StmtAst, StmtParseError, StmtParser};

#[derive(Debug, Clone)]
pub(crate) struct Block {
    inner: Vec<StmtAst>,
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

impl Runtime {
    pub(super) fn run_block(&self, block: Block) -> Result<(), RuntimeError> {
        let runtime = self.child_runtime();
        for stmt in block.inner {
            runtime.run(stmt)?;
        }
        Ok(())
    }
}
