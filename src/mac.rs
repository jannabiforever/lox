/// Wrapper for Rc::new(RefCell::new(~))
macro_rules! rc_rc {
    ($inner:expr) => {
        Rc::new(RefCell::new($inner))
    };
}

/// Implement From for wrapper enums.
macro_rules! impl_from {
    ( $target:ident : $( $variant:ident ),* ) => {
        $(
            impl From<$variant> for $target {
                fn from(value: $variant) -> Self {
                    Self::$variant(value)
                }
            }
        )*
    };
}

pub(crate) use impl_from;
pub(crate) use rc_rc;
