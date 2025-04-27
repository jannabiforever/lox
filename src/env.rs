use std::{cell::RefCell, collections::HashMap, io::Write, process::ExitCode, rc::Rc};

use crate::{
    error::{IntoLoxError, LoxError},
    function::CLOCK,
    literal::LoxValue,
    rc_rc,
};

/// Environment, which holds every variable-value bindings and reference to
/// global stdout.
pub(crate) struct Env<'a, W: Write> {
    pub(crate) stdout: Rc<RefCell<W>>,
    pub(crate) parent: Option<Rc<RefCell<Env<'a, W>>>>,
    pub(crate) scope: HashMap<String, LoxValue<'a>>,
}

impl<'a, W: Write> Env<'a, W> {
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

    /// Get value with given key. It loops through all parent scopes.
    pub fn get(&self, key: &str) -> Option<LoxValue<'a>> {
        if let Some(value) = self.scope.get(key) {
            // This value had been defined in current scope.
            Some(value.clone())
        } else if let Some(parent_env) = &self.parent {
            // This value had been defined in one of its parents' scope.
            parent_env.borrow().get(key)
        } else {
            None
        }
    }

    /// Initializes the key-value pair at current scope. It overwrites on duplicated keys.
    pub fn set(&mut self, key: &str, value: LoxValue<'a>) {
        self.scope.insert(key.to_string(), value);
    }

    /// Updates the value stored in the hashmap. If fails, returns false.
    pub fn update(&mut self, key: &str, value: LoxValue<'a>) -> bool {
        if let Some(existing_value) = self.scope.get_mut(key) {
            *existing_value = value.clone();
            true
        } else if let Some(parent_env) = &self.parent {
            parent_env.borrow_mut().update(key, value)
        } else {
            false
        }
    }

    #[inline]
    pub fn is_global(&self) -> bool {
        self.parent.is_none()
    }

    #[allow(dead_code)]
    pub fn global(env: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        if let Some(parent) = env.borrow().parent.as_ref() {
            Self::global(parent)
        } else {
            env.clone()
        }
    }

    pub fn capture(&self) -> HashMap<String, LoxValue<'a>> {
        let mut start = self.scope.clone();
        if let Some(parent_env) = self.parent.clone() {
            let capture_of_parent = parent_env.borrow().capture();
            for (k, v) in capture_of_parent {
                if !start.contains_key(&k) {
                    start.insert(k, v);
                }
            }
        }
        start
    }
}

/// Trait for eval expressions.
pub(crate) trait Evaluatable<'a> {
    // Required methods
    fn eval<W: Write>(
        &self,
        env: Rc<RefCell<Env<'a, W>>>,
    ) -> Result<LoxValue<'a>, LoxError<RuntimeError>>;

    /// Every evaluatable could return Err(RuntimeError).
    /// To report errors generously, we need to know where.
    fn line(&self) -> usize;
}

/// Trait for run statements.
pub(crate) trait Runnable<'a> {
    // Required methods
    fn run<W: Write>(
        &self,
        env: Rc<RefCell<Env<'a, W>>>,
    ) -> Result<Option<LoxValue<'a>>, LoxError<RuntimeError>>;

    /// Every runnable could return Err(RuntimeError).
    /// To report errors generously, we need to know where.
    fn line(&self) -> usize;
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

    #[error("Error: Cannot return at global scope.")]
    ReturnAtGlobal,
}

impl IntoLoxError for RuntimeError {
    fn exit_code(&self) -> ExitCode {
        ExitCode::from(70)
    }
}
