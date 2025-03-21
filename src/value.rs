pub enum Value {
    Number(Number),
    String(String),
    Boolean(bool),
    Nil,
}

pub struct Number(f64);

impl std::str::FromStr for Number {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == self.0.trunc() {
            // No integer-like formatting for floats.
            write!(f, "{}.0", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
