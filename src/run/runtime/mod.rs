mod print;
mod var_decl;

use std::{cell::RefCell, rc::Rc};

use crate::{
    evaluate::{Environment, Evaluator},
    literal::Literal,
    parse::ExprAst,
};

use super::{stmt_ast::StmtAst, RuntimeError};

pub struct Runtime {
    global_env: Rc<RefCell<Environment>>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            global_env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn run(&self, stmt: StmtAst) -> Result<(), RuntimeError> {
        match stmt {
            StmtAst::Print(print) => self.run_print(print)?,
            StmtAst::Expression(expr) => {
                // Evaluate the expression, but ignore the result.
                self.evaluate(&expr.expr)?;
            }
            StmtAst::VarDecl(var_decl) => self.run_var_decl(var_decl)?,
        }

        Ok(())
    }

    fn evaluate(&self, expr: &ExprAst) -> Result<Literal, RuntimeError> {
        self.evaluator().eval(expr).map_err(Into::into)
    }

    fn evaluator(&self) -> Evaluator {
        Evaluator::with_env(self.global_env.clone())
    }
}
