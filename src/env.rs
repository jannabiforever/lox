use std::{cell::RefCell, collections::HashMap, process::ExitCode, rc::Rc};

use crate::{error::IntoLoxError, literal::Literal, rc_rc};

pub(crate) struct Env {
    pub(crate) parent: Option<Rc<RefCell<Env>>>,
    pub(crate) scope: HashMap<String, Literal>,
}

impl Env {
    /// Creates a global environment,
    pub fn new() -> Rc<RefCell<Self>> {
        rc_rc!(Self {
            parent: None,
            scope: HashMap::new(),
        })
    }

    /// New child environment instance.
    pub fn from_parent(parent: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        rc_rc!(Self {
            parent: Some(parent),
            scope: HashMap::new(),
        })
    }

    pub fn get(&self, key: &str) -> Option<Literal> {
        if let Some(value) = self.scope.get(key) {
            Some(value.clone())
        } else if let Some(parent_env) = &self.parent {
            parent_env.borrow().get(key)
        } else {
            None
        }
    }

    /// Initializes the key-value pair at current scope.
    pub fn set(&mut self, key: &str, value: Literal) {
        self.scope.insert(key.to_string(), value);
    }

    /// Updates the value stored in the hashmap. If fails, returns false.
    pub fn update(&mut self, key: &str, value: Literal) -> bool {
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
    fn eval(&self, env: Rc<RefCell<Env>>) -> Result<Literal, EvaluateError>;
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
