mod env;
mod operate;

use env::Environment;

use crate::{
    error::{LoxError, WithLine},
    expr_ast::{Binary, Expr, Grouping, Unary},
    literal::Literal,
};

pub fn evaluate_single_expr_ast(source: &str) -> WithLine<Result<Literal, LoxError>> {
    let (line, expr) = crate::expr_ast::parse_expr_ast(source).into();
    let mut env = Environment::new();
    WithLine::new(line, evaluate(&expr.unwrap(), &mut env))
}

pub fn evaluate(expr: &Expr, env: &mut Environment) -> Result<Literal, LoxError> {
    match expr {
        Expr::Binary(Binary { left, op, right }) => {
            let func = operate::binary_function(*op);
            let left = evaluate(left, env)?;
            let right = evaluate(right, env)?;

            func(left, right).map_err(|e| e.into())
        }
        Expr::Literal(v) => Ok(v.clone()),
        Expr::Unary(Unary { op, right }) => {
            let func = operate::unary_function(*op);
            let right = evaluate(right, env)?;

            func(right).map_err(|e| e.into())
        }
        Expr::Grouping(Grouping { inner }) => evaluate(inner, env),
    }
}
