#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BindingPower {
    #[default]
    None,
    Unary,
}
