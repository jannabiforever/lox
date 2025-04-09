use std::{
    cell::RefCell,
    io::Write,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    env::RuntimeError,
    literal::{Literal, LoxValue, Number},
    Env,
};

pub(crate) trait Callable {
    // Required methods
    fn argument_names(&self) -> &[&str];
    fn run_body<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<LoxValue, RuntimeError>;

    // Provided methods
    fn call<W: Write>(
        &self,
        arguments: Vec<LoxValue>,
        env: Rc<RefCell<Env<W>>>,
    ) -> Result<LoxValue, RuntimeError> {
        if self.arity() != arguments.len() {
            return Err(RuntimeError::InvalidNumberOfArguments);
        }

        let env = self.stack_scope(arguments, env);
        self.run_body(env)
    }

    fn arity(&self) -> usize {
        self.argument_names().len()
    }

    fn stack_scope<W: Write>(
        &self,
        arguments: Vec<LoxValue>,
        env: Rc<RefCell<Env<W>>>,
    ) -> Rc<RefCell<Env<W>>> {
        let new_env = Env::from_parent(env);
        for (key, value) in self.argument_names().iter().zip(arguments.into_iter()) {
            new_env.borrow_mut().set(key, value);
        }
        new_env
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RustFunction {
    pub(crate) name: &'static str,
    pub(crate) arguments: Vec<&'static str>,
}

pub(crate) static CLOCK: LoxValue = LoxValue::RustFunction(RustFunction {
    name: "clock",
    arguments: vec![],
});

impl Callable for RustFunction {
    fn argument_names(&self) -> &[&str] {
        &self.arguments
    }

    fn run_body<W: Write>(&self, _: Rc<RefCell<Env<W>>>) -> Result<LoxValue, RuntimeError> {
        match self.name {
            "clock" => Ok(clock()),
            rest => unreachable!("there are no builtin function named {rest}"),
        }
    }
}

fn clock() -> LoxValue {
    let elapsed_secs_from_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as f64;

    LoxValue::Literal(Literal::Number(Number(elapsed_secs_from_epoch)))
}
