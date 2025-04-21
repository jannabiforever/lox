use std::{
    cell::RefCell,
    collections::HashMap,
    fmt,
    io::Write,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    env::{
        Evaluatable, Runnable,
        RuntimeError::{self, *},
    },
    literal::{Literal, LoxValue, Number},
    statement::{Return, StmtAst},
    Env,
};

pub(crate) trait Callable<'a> {
    // Required methods
    fn argument_names(&self) -> Vec<&str>;

    /// call and get the result.
    fn call<W: Write>(
        &self,
        arguments: Vec<LoxValue<'a>>,
        env: Rc<RefCell<Env<'a, W>>>,
    ) -> Result<LoxValue<'a>, RuntimeError>;
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

    fn call<W: Write>(
        &self,
        arguments: Vec<LoxValue<'a>>,
        _: Rc<RefCell<Env<'a, W>>>,
    ) -> Result<LoxValue<'a>, RuntimeError> {
        if arguments.len() != self.arguments.len() {
            return Err(InvalidNumberOfArguments);
        }

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

impl<'a> LoxFunction<'a> {
    // TODO: How should captured variables be handled?
    fn stack_scope_with_captured<W: Write>(
        &self,
        arguments: Vec<LoxValue<'a>>,
        env: Rc<RefCell<Env<'a, W>>>,
    ) -> Rc<RefCell<Env<'a, W>>> {
        let env = Env::from_parent(env);
        for (key, value) in self.argument_names().iter().zip(arguments.into_iter()) {
            env.borrow_mut().set(key, value);
        }
        for (k, v) in self.captured.iter() {
            env.borrow_mut().set(k, v.clone());
        }
        env
    }
}

impl<'a> Callable<'a> for LoxFunction<'a> {
    fn argument_names(&self) -> Vec<&str> {
        self.arguments.iter().map(|s| s.as_str()).collect()
    }

    fn call<W: Write>(
        &self,
        arguments: Vec<LoxValue<'a>>,
        env: Rc<RefCell<Env<'a, W>>>,
    ) -> Result<LoxValue<'a>, RuntimeError> {
        if arguments.len() != self.arguments.len() {
            return Err(InvalidNumberOfArguments);
        }

        let env = self.stack_scope_with_captured(arguments, env);
        for stmt in self.body.iter() {
            match stmt {
                StmtAst::Return(Return { expr, .. }) => {
                    let value = expr
                        .as_ref()
                        .map(|e| e.eval(env))
                        .transpose()
                        // when called, error line should be not from the function body
                        .map_err(|err| err.kind)?
                        .unwrap_or_default();

                    return Ok(value);
                }
                rest => {
                    // when called, error line should be not from the function body
                    if let Some(value) = rest.run(env.clone()).map_err(|err| err.kind)? {
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
