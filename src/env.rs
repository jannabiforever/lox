use std::{cell::RefCell, collections::HashMap, io::Write, process::ExitCode, rc::Rc};

use crate::{error::IntoLoxError, literal::LoxValue, rc_rc, statement::RuntimeError};

pub(crate) struct Env<W: Write> {
    pub(crate) stdout: Rc<RefCell<W>>,
    pub(crate) parent: Option<Rc<RefCell<Env<W>>>>,
    pub(crate) scope: HashMap<String, LoxValue>,
}

impl<W: Write> Env<W> {
    /// Creates a global environment,
    pub fn new(stdout: W) -> Rc<RefCell<Self>> {
        rc_rc!(Self {
            stdout: rc_rc!(stdout),
            parent: None,
            scope: HashMap::new(),
        })
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
    fn eval<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<LoxValue, EvaluateError>;
}

#[derive(Debug, thiserror::Error, Clone)]
pub(crate) enum EvaluateError {
    #[error("Error: Operand must be {0}")]
    OperandMustBe(&'static str),

    #[error("Error: Undefined variable '{0}'.")]
    UndefinedVariable(String),

    #[error("Error: Cannot assign value into '{0}'.")]
    InvalidAssignmentTarget(String),

    #[error("Error: Cannot call '{0}'")]
    InvalidCallTarget(String),
}

impl IntoLoxError for EvaluateError {
    fn exit_code(&self) -> ExitCode {
        ExitCode::from(65)
    }
}

pub(crate) trait Runnable {
    // Required methods
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<(), RuntimeError>;
}
