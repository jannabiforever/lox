mod env;

use crate::{
    error::{LoxError, WithLine},
    value::Literal,
};

pub fn evaluate_single_expr_ast(source: &str) -> WithLine<Result<Literal, LoxError>> {
    todo!()
}
