use std::{cell::RefCell, collections::HashMap, io::Write, process::ExitCode, rc::Rc};

use crate::{
    error::{IntoLoxError, LoxError},
    function::rust_clock_function,
    literal::LoxValue,
    rc_rc,
};

/// Environment, which holds every variable-value bindings and reference to
/// global stdout.
pub(crate) struct Env<'src> {
    pub(crate) parent: Option<Rc<RefCell<Env<'src>>>>,
    pub(crate) scope: HashMap<String, LoxValue<'src>>,
}

impl<'src> Env<'src> {
    /// Creates a global environment,
    pub fn new() -> Rc<RefCell<Self>> {
        let env = rc_rc!(Self {
            parent: None,
            scope: HashMap::new()
        });

        env.borrow_mut().set("clock", rust_clock_function().into());
        env
    }

    /// New child environment instance.
    pub fn from_parent(parent: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        rc_rc!(Self {
            parent: Some(parent.clone()),
            scope: HashMap::new(),
        })
    }

    /// Get value with given key. It loops through all parent scopes.
    pub fn get(&self, key: &str) -> Option<LoxValue<'src>> {
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

    /// Initializes the key-value pair at current scope. It overwrites on
    /// duplicated keys.
    pub fn set(&mut self, key: &str, value: LoxValue<'src>) {
        self.scope.insert(key.to_string(), value);
    }

    /// Updates the value stored in the hashmap. If fails, returns false.
    pub fn update(&mut self, key: &str, value: LoxValue<'src>) -> bool {
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
        self.parent.is_none() // && self.depth == 0?
    }
}

/// Trait for eval expressions.
pub(crate) trait Evaluatable<'a> {
    // Required methods
    fn eval<W: Write>(
        &self,
        env: Rc<RefCell<Env<'a>>>,
        stdout: &mut W,
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
        env: Rc<RefCell<Env<'a>>>,
        stdout: &mut W,
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
