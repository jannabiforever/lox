mod print;

use crate::{evaluate::Evaluator, literal::Literal, parse::ExprAst};

use super::{stmt_ast::StmtAst, RuntimeError};

pub struct Runtime;

impl Runtime {
    pub fn new() -> Self {
        Runtime
    }

    pub fn run(&mut self, stmt: StmtAst) -> Result<(), RuntimeError> {
        match stmt {
            StmtAst::Print(print) => {
                self.run_print(print)?;
            }
            StmtAst::Expression(expr) => {
                // Evaluate the expression, but ignore the result.
                self.evaluate(&expr.expr)?;
            }
        }

        Ok(())
    }

    fn evaluate(&self, expr: &ExprAst) -> Result<Literal, RuntimeError> {
        self.evaluator().eval(expr).map_err(Into::into)
    }

    fn evaluator(&self) -> Evaluator {
        Evaluator::new()
    }
}
