use crate::{
    parse::ExprAst,
    run::{stmt_ast::VarDecl, RuntimeError},
};

use super::Runtime;

impl Runtime {
    pub(super) fn run_var_decl(&self, var_decl: VarDecl) -> Result<(), RuntimeError> {
        let VarDecl { var, value } = var_decl;

        let var = match var {
            ExprAst::Variable(var) => Ok(var),
            rest => Err(RuntimeError::InvalidAssignmentTarget(rest.to_string())),
        }?;

        let value = self.evaluate(&value)?;

        self.global_env.borrow_mut().update(var, value)?;

        Ok(())
    }
}
