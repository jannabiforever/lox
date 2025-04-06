use super::{Runtime, RuntimeError, StmtAst};

#[derive(Debug, Clone)]
pub(crate) struct Block {
    inner: Vec<StmtAst>,
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
