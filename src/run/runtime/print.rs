use crate::run::{stmt_ast::Print, RuntimeError};

use super::Runtime;

impl Runtime {
    pub fn run_print(&mut self, print: Print) -> Result<(), RuntimeError> {
        let value = self.evaluate(&print.expr)?;
        println!("{}", value);
        Ok(())
    }
}
