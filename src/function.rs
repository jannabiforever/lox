#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RustFunction {
    pub(crate) name: &'static str,
    pub(crate) arguments: Vec<&'static str>,
}

impl RustFunction {
    pub(crate) fn arity(&self) -> usize {
        self.arguments.len()
    }
}

pub(crate) static CLOCK: RustFunction = RustFunction {
    name: "clock",
    arguments: vec![],
};
