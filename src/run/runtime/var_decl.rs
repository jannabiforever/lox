use crate::run::{stmt_ast::VarDecl, RuntimeError};

use super::Runtime;

impl Runtime {
    pub(super) fn run_var_decl(&self, var_decl: VarDecl) -> Result<(), RuntimeError> {
        let VarDecl { var, value } = var_decl;
        let var = self.assignable_key(&var)?;
        let value = self.evaluate(&value)?;
        self.global_env.borrow_mut().set(&var, value);
        Ok(())
    }
}
