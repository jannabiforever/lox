use crate::{literal::Literal, stmt_ast::Stmt};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

macro_rules! rc_rc {
    ($e:expr) => {
        Rc::new(RefCell::new($e))
    };
}

pub(super) struct Env {
    global: Option<Rc<RefCell<Env>>>,
    local_literal_map: RefCell<HashMap<String, Literal>>,
    local_callable_map: RefCell<HashMap<String, Vec<Stmt>>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            global: None,
            local_literal_map: RefCell::new(HashMap::new()),
            local_callable_map: RefCell::new(HashMap::new()),
        }
    }

    pub fn new_with_parent(parent: Rc<RefCell<Env>>) -> Self {
        Self {
            global: Some(parent),
            local_literal_map: RefCell::new(HashMap::new()),
            local_callable_map: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_literal(&self, key: &str) -> Option<Literal> {
        // find locally first.
        if let Some(val) = self.local_literal_map.borrow().get(key) {
            return Some(val.clone());
        }
        // find globally
        match &self.global {
            Some(g) => g.borrow().get_literal(key),
            None => None,
        }
    }

    #[allow(dead_code)]
    pub fn get_callable(&self, key: &str) -> Option<Vec<Stmt>> {
        // find locally first.
        if let Some(val) = self.local_callable_map.borrow().get(key) {
            return Some(val.clone());
        }
        // find globally
        match &self.global {
            Some(g) => g.borrow().get_callable(key),
            None => None,
        }
    }

    pub fn local_literal_insert(&mut self, key: &str, val: Literal) {
        self.local_literal_map.borrow_mut().insert(key.into(), val);
    }

    pub fn local_callable_insert(&mut self, key: &str, val: Vec<Stmt>) {
        self.local_callable_map.borrow_mut().insert(key.into(), val);
    }

    /// This might fail if not declared before.
    /// Returns false if failed.
    pub fn global_literal_insert(&self, key: &str, val: Literal) -> bool {
        {
            let mut local_map = self.local_literal_map.borrow_mut();
            if local_map.contains_key(key) {
                local_map.insert(key.to_string(), val);
                return true;
            }
        }

        match &self.global {
            Some(g) => g.borrow().global_literal_insert(key, val),
            None => false,
        }
    }

    #[allow(dead_code)]
    /// This might fail if not declared before.
    /// Returns false if failed.
    pub fn global_callable_insert(&self, key: &str, val: Vec<Stmt>) -> bool {
        {
            let mut local_map = self.local_callable_map.borrow_mut();
            if local_map.contains_key(key) {
                local_map.insert(key.to_string(), val);
                return true;
            }
        }

        match &self.global {
            Some(g) => g.borrow().global_callable_insert(key, val),
            None => false,
        }
    }
}

pub(crate) use rc_rc;
