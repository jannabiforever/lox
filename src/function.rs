use std::{
    cell::RefCell,
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
    statement::{FunctionDef, Return, StmtAst},
    Env,
};

pub(crate) trait Callable<'a> {
    // Required methods
    fn argument_names(&self) -> Vec<&str>;

    /// call and get the result.
    fn call<W: Write>(
        &self,
        arguments: Vec<LoxValue<'a>>,
        env: Rc<RefCell<Env<'a>>>,
        stdout: &mut W,
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
        _: Rc<RefCell<Env<'a>>>,
        _: &mut W,
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
    pub(crate) def: FunctionDef<'a>,
}

impl<'a> Callable<'a> for LoxFunction<'a> {
    fn argument_names(&self) -> Vec<&str> {
        self.def.arguments.iter().map(|s| s.as_str()).collect()
    }

    fn call<W: Write>(
        &self,
        arguments: Vec<LoxValue<'a>>,
        env: Rc<RefCell<Env<'a>>>,
        stdout: &mut W,
    ) -> Result<LoxValue<'a>, RuntimeError> {
        if arguments.len() != self.def.arguments.len() {
            return Err(InvalidNumberOfArguments);
        }

        // Initialize scope environment.
        let scope_env = Env::from_parent(env);
        for (name, value) in self.argument_names().into_iter().zip(arguments.into_iter()) {
            scope_env.borrow_mut().set(name, value);
        }

        for stmt in self.def.body.iter() {
            match stmt {
                StmtAst::Return(Return { expr, .. }) => {
                    let value = expr
                        .as_ref()
                        .map(|e| e.eval(scope_env, stdout))
                        .transpose()
                        // when called, error line should be not from the function body
                        .map_err(|err| err.kind)?
                        .unwrap_or_default();

                    return Ok(value);
                }
                rest => {
                    // when called, error line should be not from the function body
                    if let Some(value) = rest
                        .run(scope_env.clone(), stdout)
                        .map_err(|err| err.kind)?
                    {
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
        write!(f, "<fn {}>", self.def.name)
    }
}
