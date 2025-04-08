use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RustFunction {
    pub(crate) name: &'static str,
    pub(crate) arguments: Vec<&'static str>,
}

pub(crate) static CLOCK: LazyLock<RustFunction> = LazyLock::new(|| RustFunction {
    name: "clock",
    arguments: vec![],
});
