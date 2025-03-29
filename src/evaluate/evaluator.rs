use crate::{literal::Literal, parse::ExprAst};

pub(crate) struct Evaluator;

impl Evaluator {
    pub fn eval(&self, expr_ast: &ExprAst) -> Literal {
        match expr_ast {
            ExprAst::Assign(assign) => todo!("self.evaluate_assign(assign)"),
            ExprAst::Binary(binary) => self.evaluate_binary(binary),
            ExprAst::FieldCall(field_call) => todo!("self.evaluate_field_call(field_call)"),
            ExprAst::FunctionCall(function_call) => {
                todo!("self.evaluate_function_call(function_call)")
            }
            ExprAst::Grouping(grouping) => {
                let inner = grouping.inner.as_ref();
                self.eval(inner)
            }
            ExprAst::Literal(literal) => literal.clone(),
            ExprAst::Unary(unary) => self.evaluate_unary(unary),
            ExprAst::Variable(variable_name) => todo!("self.evaluate_variable(variable_name)"),
        }
    }
}
