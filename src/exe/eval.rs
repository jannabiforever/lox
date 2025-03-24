use std::{cell::RefCell, rc::Rc};

use super::{
    env::{Env, rc_rc},
    operate::{binary_function, unary_function},
};
use crate::{
    error::{LoxError, RuntimeError, WithLine},
    expr_ast::{Assign, Binary, Expr, Grouping, Unary},
    literal::Literal,
};

pub fn evaluate_single_expr_ast(source: &str) -> Result<Literal, WithLine<LoxError>> {
    let expr_ast_with_line: Result<WithLine<Expr>, WithLine<LoxError>> =
        crate::expr_ast::parse_expr_ast(source).into();
    let env = rc_rc!(Env::new());

    let (line, expr) = expr_ast_with_line?.split();
    eval_expr(&expr, env).map_err(|e| WithLine::new(line, e.into()))
}

pub(super) fn eval_expr(expr: &Expr, env: Rc<RefCell<Env>>) -> Result<Literal, RuntimeError> {
    match expr {
        Expr::Assign(Assign { variable, value }) => {
            let value = eval_expr(value, env.clone())?;
            env.borrow_mut()
                .global_literal_insert(&variable.source, value.clone());
            Ok(value)
        }
        Expr::Binary(Binary { left, op, right }) => {
            let func = binary_function(*op);
            let left = eval_expr(left, env.clone())?;
            let right = eval_expr(right, env)?;

            func(left, right)
        }
        Expr::Grouping(Grouping { inner }) => eval_expr(inner, env),
        Expr::Literal(v) => Ok(v.clone()),
        Expr::Unary(Unary { op, right }) => {
            let func = unary_function(*op);
            let right = eval_expr(right, env)?;

            func(right).map_err(|e| e.into())
        }
        Expr::Variable(v) => env
            .borrow()
            .get_literal(&v.name.source)
            .ok_or_else(|| RuntimeError::UndefinedVar(v.name.source.clone())),
    }
}
