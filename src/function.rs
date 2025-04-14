use std::cell::RefCell;
use std::fmt;
use std::io::Write;
use std::rc::Rc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::env::RuntimeError;
use crate::literal::Literal;
use crate::literal::LoxValue;
use crate::literal::Number;
use crate::statement::Return;
use crate::statement::StmtAst;
use crate::Env;
use crate::Evaluatable;
use crate::Runnable;

pub(crate) trait Callable {
    // Required methods
    fn argument_names(&self) -> Vec<&str>;
    fn run_body<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<LoxValue, RuntimeError>;

    // Provided methods

    /// call and get the result.
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

    /// Create new scope with having current env as own parent, and assign
    /// given function arguments into this env.
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

impl fmt::Display for RustFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}

pub(crate) static CLOCK: LoxValue = LoxValue::RustFunction(RustFunction {
    name: "clock",
    arguments: vec![],
});

impl Callable for RustFunction {
    fn argument_names(&self) -> Vec<&str> {
        self.arguments.iter().map(|&s| s).collect()
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LoxFunction {
    pub(crate) name: String,
    pub(crate) arguments: Vec<String>,
    pub(crate) body: Vec<StmtAst>,
}

impl Callable for LoxFunction {
    fn argument_names(&self) -> Vec<&str> {
        self.arguments.iter().map(|s| s.as_str()).collect()
    }

    fn run_body<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<LoxValue, RuntimeError> {
        for stmt in self.body.iter() {
            match stmt {
                StmtAst::Return(Return { expr: inner }) => {
                    let value = inner.eval(env)?;
                    return Ok(value);
                }
                rest => {
                    if let Some(value) = rest.run(env.clone())? {
                        return Ok(value);
                    }
                }
            }
        }

        Ok(LoxValue::Literal(Literal::Nil))
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}
