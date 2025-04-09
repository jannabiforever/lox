use std::time::{SystemTime, UNIX_EPOCH};

use crate::literal::{Literal, LoxValue, Number};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RustFunction {
    pub(crate) name: &'static str,
    pub(crate) arguments: Vec<&'static str>,
}

pub(crate) static CLOCK: LoxValue = LoxValue::RustFunction(RustFunction {
    name: "clock",
    arguments: vec![],
});

impl RustFunction {
    pub(crate) fn call(&self, arguments: Vec<LoxValue>) -> LoxValue {
        if self.arity() != arguments.len() {
            panic!("Number of given arguments is invalid.")
        }
        match self.name {
            "clock" => clock(),
            rest => {
                panic!("There is no builtin function named {rest}")
            }
        }
    }

    pub(crate) fn arity(&self) -> usize {
        self.arguments.len()
    }
}

fn clock() -> LoxValue {
    let elapsed_secs_from_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as f64;

    LoxValue::Literal(Literal::Number(Number(elapsed_secs_from_epoch)))
}
