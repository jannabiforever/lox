use super::{stmt_ast::StmtAst, RuntimeError};

pub struct Runtime {
    stmts: Vec<StmtAst>,
}

impl Runtime {
    pub fn new(stmts: Vec<StmtAst>) -> Self {
        Runtime { stmts }
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        for stmt in &self.stmts {
            todo!()
        }

        Ok(())
    }
}
