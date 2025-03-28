pub trait Operator {
    fn binding_power(&self) -> BindingPower;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BindingPower {
    #[default]
    None,
    Bang,
    UnaryMinus,
}
