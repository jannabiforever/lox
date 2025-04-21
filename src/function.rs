use std::{
    cell::RefCell,
    collections::HashMap,
    fmt,
    io::Write,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    env::{Evaluatable, Runnable, RuntimeError},
    error::{IntoLoxError, LoxError},
    literal::{Literal, LoxValue, Number},
    statement::{Return, StmtAst},
    Env,
};

pub(crate) trait Callable<'a> {
    // Required methods
    fn argument_names(&self) -> Vec<&str>;
    fn run_body<W: Write>(
        &self,
        env: Rc<RefCell<Env<'a, W>>>,
    ) -> Result<LoxValue<'a>, LoxError<RuntimeError>>;

    // Provided methods

    /// call and get the result.
    fn call<W: Write>(
        &self,
        arguments: Vec<LoxValue<'a>>,
        env: Rc<RefCell<Env<'a, W>>>,
    ) -> Result<LoxValue<'a>, LoxError<RuntimeError>> {
        if self.arity() != arguments.len() {
            #[allow(unreachable_code, clippy::diverging_sub_expression)] // TODO
            return Err(RuntimeError::InvalidNumberOfArguments.at(todo!("Get error line")));
        }

        let env = self.stack_scope(arguments, env);
        self.run_body(env)
    }

    fn arity(&self) -> usize {
        self.argument_names().len()
    }

    /// Create new scope with having current env as own parent, and assign
    /// given function arguments into this env.
    fn stack_scope<W: Write>(
        &self,
        arguments: Vec<LoxValue<'a>>,
        env: Rc<RefCell<Env<'a, W>>>,
    ) -> Rc<RefCell<Env<'a, W>>> {
        let new_env = Env::from_parent(env);
        // Assign arguments to the scope environment.
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

impl fmt::Display for RustFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}

pub(crate) static CLOCK: LoxValue = LoxValue::RustFunction(RustFunction {
    name: "clock",
    arguments: vec![],
});

impl<'a> Callable<'a> for RustFunction {
    fn argument_names(&self) -> Vec<&str> {
        self.arguments.to_vec()
    }

    fn run_body<W: Write>(
        &self,
        _: Rc<RefCell<Env<'a, W>>>,
    ) -> Result<LoxValue<'a>, LoxError<RuntimeError>> {
        match self.name {
            "clock" => Ok(clock()),
            rest => unreachable!("there are no builtin function named {rest}"),
        }
    }
}

fn clock<'a>() -> LoxValue<'a> {
    let elapsed_secs_from_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as f64;

    LoxValue::Literal(Literal::Number(Number(elapsed_secs_from_epoch)))
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LoxFunction<'a> {
    pub(crate) name: String,
    pub(crate) arguments: Vec<String>,
    pub(crate) body: Vec<StmtAst<'a>>,
    pub(crate) captured: HashMap<String, LoxValue<'a>>,
}

impl<'a> Callable<'a> for LoxFunction<'a> {
    fn argument_names(&self) -> Vec<&str> {
        self.arguments.iter().map(|s| s.as_str()).collect()
    }

    fn run_body<W: Write>(
        &self,
        env: Rc<RefCell<Env<'a, W>>>,
    ) -> Result<LoxValue<'a>, LoxError<RuntimeError>> {
        let env = Env::from_parent(env);
        for (k, v) in self.captured.iter() {
            env.borrow_mut().set(k, v.clone());
        }
        for stmt in self.body.iter() {
            match stmt {
                StmtAst::Return(Return { expr, .. }) => {
                    let value = expr
                        .as_ref()
                        .map(|e| e.eval(env))
                        .transpose()?
                        .unwrap_or_default();

                    return Ok(value);
                }
                rest => {
                    if let Some(value) = rest.run(env.clone())? {
                        return Ok(value);
                    }
                }
            }
        }

        Ok(LoxValue::default())
    }
}

impl fmt::Display for LoxFunction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}
