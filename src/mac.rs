/// Wrapper for Rc::new(RefCell::new(~))
macro_rules! rc_rc {
    ($inner:expr) => {
        Rc::new(RefCell::new($inner))
    };
}

pub(crate) use rc_rc;
