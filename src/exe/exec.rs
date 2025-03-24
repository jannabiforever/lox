use std::{cell::RefCell, rc::Rc};

use super::{
    env::{Env, rc_rc},
    eval::eval_expr,
};
use crate::{
    error::{LoxError, RuntimeError, WithLine},
    literal::Literal,
    stmt_ast::{Stmt, parse_stmt_ast},
};

pub fn execute_stmt_ast(source: &str) -> Result<(), WithLine<LoxError>> {
    let stmts_result_with_line = parse_stmt_ast(source);
    let env = rc_rc!(Env::new());
    for stmt_result_with_line in stmts_result_with_line {
        let stmt_result_with_line: Result<WithLine<Stmt>, WithLine<LoxError>> =
            stmt_result_with_line.into();
        let (line, stmt) = stmt_result_with_line?.split();

        exec_stmt(&stmt, env.clone()).map_err(|e| WithLine::new(line, e.into()))?;
    }

    Ok(())
}

fn exec_stmt(stmt: &Stmt, env: Rc<RefCell<Env>>) -> Result<(), RuntimeError> {
    match stmt {
        Stmt::PrintStmt(stmt) => {
            let value = eval_expr(&stmt.expr, env)?;
            println!("{}", value);
        }
        Stmt::ExprStmt(stmt) => {
            eval_expr(&stmt.expr, env)?;
        }
        Stmt::WhileStmt(stmt) => {
            while eval_expr(&stmt.condition, env.clone())?.is_truthy() {
                exec_stmt(&stmt.body, env.clone())?;
            }
        }
        Stmt::IfStmt(stmt) => {
            if eval_expr(&stmt.condition, env.clone())?.is_truthy() {
                exec_stmt(&stmt.then_branch, env.clone())?;
            } else if let Some(else_branch) = &stmt.else_branch {
                exec_stmt(else_branch, env.clone())?;
            }
        }
        Stmt::ForStmt(stmt) => {
            if let Some(initializer) = &stmt.initializer {
                exec_stmt(&initializer, env.clone())?;
            }

            while stmt
                .condition
                .as_ref()
                .map(|c| eval_expr(c, env.clone()).map(|v| v.is_truthy()))
                .unwrap_or(Ok(true))?
            {
                // Execute the body
                exec_stmt(&stmt.body, env.clone())?;

                // then increment
                if let Some(increment) = &stmt.increment {
                    eval_expr(increment, env.clone())?;
                }
            }
        }
        Stmt::FuncDeclStmt(stmt) => {
            env.borrow_mut()
                .local_callable_insert(&stmt.name, stmt.clone().into());
        }
        Stmt::ReturnStmt(_) => {
            // Every return statement should be inside a function definition.
            return Err(RuntimeError::ReturnFromTopLevel);
        }
        Stmt::BlockStmt(stmt) => {
            let new_env = rc_rc!(Env::new_with_parent(env));
            for stmt in &stmt.stmts {
                exec_stmt(stmt, new_env.clone())?;
            }
        }
        Stmt::VarDeclStmt(stmt) => {
            let value = if let Some(initializer) = &stmt.initializer {
                // e.g. var x = 1;
                eval_expr(initializer, env.clone())?
            } else {
                Literal::Nil
            };

            env.borrow_mut().local_literal_insert(&stmt.name, value);
        }
    }

    Ok(())
}
