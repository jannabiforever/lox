use std::{cell::RefCell, collections::HashMap, io::Write, process::ExitCode, rc::Rc};

use crate::{error::IntoLoxError, function::CLOCK, literal::LoxValue, rc_rc};

pub(crate) struct Env<W: Write> {
    pub(crate) stdout: Rc<RefCell<W>>,
    pub(crate) parent: Option<Rc<RefCell<Env<W>>>>,
    pub(crate) scope: HashMap<String, LoxValue>,
}

impl<W: Write> Env<W> {
    /// Creates a global environment,
    pub fn new(stdout: W) -> Rc<RefCell<Self>> {
        let env = rc_rc!(Self {
            stdout: rc_rc!(stdout),
            parent: None,
            scope: HashMap::new()
        });

        env.borrow_mut().set("clock", CLOCK.clone());
        env
    }

    /// New child environment instance.
    pub fn from_parent(parent: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        rc_rc!(Self {
            stdout: parent.borrow().stdout.clone(),
            parent: Some(parent.clone()),
            scope: HashMap::new(),
        })
    }

    pub fn get(&self, key: &str) -> Option<LoxValue> {
        if let Some(value) = self.scope.get(key) {
            Some(value.clone())
        } else if let Some(parent_env) = &self.parent {
            parent_env.borrow().get(key)
        } else {
            None
        }
    }

    /// Initializes the key-value pair at current scope.
    pub fn set(&mut self, key: &str, value: LoxValue) {
        self.scope.insert(key.to_string(), value);
    }

    /// Updates the value stored in the hashmap. If fails, returns false.
    pub fn update(&mut self, key: &str, value: LoxValue) -> bool {
        if let Some(existing_value) = self.scope.get_mut(key) {
            *existing_value = value.clone();
            true
        } else if let Some(parent_env) = &self.parent {
            parent_env.borrow_mut().update(key, value)
        } else {
            false
        }
    }
}

pub(crate) trait Evaluatable {
    // Required methods
    fn eval<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<LoxValue, RuntimeError>;
}

pub(crate) trait Runnable {
    // Required methods
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<(), RuntimeError>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum RuntimeError {
    #[error("Error: Operand must be {0}")]
    OperandMustBe(&'static str),

    #[error("Error: Undefined variable '{0}'.")]
    UndefinedVariable(String),

    #[error("Error: Cannot call '{0}'")]
    InvalidCallTarget(String),

    #[error("Error: Invalid number of arguments")]
    InvalidNumberOfArguments,

    #[error("Error: Cannot assign value into '{0}'.")]
    InvalidAssignmentTarget(String),
}

impl IntoLoxError for RuntimeError {
    fn exit_code(&self) -> ExitCode {
        ExitCode::from(70)
    }
}
