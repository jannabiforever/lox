use std::{cell::RefCell, rc::Rc};

use crate::{
    literal::Literal,
    parse::{ExprAst, Grouping},
};

use super::{error::EvaluateError, Environment};

pub(crate) struct Evaluator {
    pub(super) env: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn with_env(env: Rc<RefCell<Environment>>) -> Self {
        Evaluator { env }
    }

    pub fn eval(&self, expr_ast: &ExprAst) -> Result<Literal, EvaluateError> {
        match expr_ast {
            ExprAst::Assign(assign) => self.evaluate_assign(assign),
            ExprAst::Binary(binary) => self.evaluate_binary(binary),
            ExprAst::FieldCall(_) => todo!("self.evaluate_field_call(field_call)"),
            ExprAst::FunctionCall(_) => {
                todo!("self.evaluate_function_call(function_call)")
            }
            ExprAst::Grouping(Grouping { inner }) => {
                let inner = inner.as_ref();
                self.eval(inner)
            }
            ExprAst::Literal(literal) => Ok(literal.clone()),
            ExprAst::Unary(unary) => self.evaluate_unary(unary),
            ExprAst::Variable(variable) => self.evaluate_variable(variable),
        }
    }
}
