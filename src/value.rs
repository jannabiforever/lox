use std::fmt;

#[derive(Debug, Clone)]
pub enum Literal {
    Number(Number),
    String(String),
    Boolean(bool),
    Nil,
}

impl Literal {
    pub fn number_from_source(source: String) -> Self {
        Self::Number(source.parse().unwrap())
    }

    pub fn string_from_source(source: String) -> Self {
        Self::String(source.trim_matches('"').to_string())
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Boolean(v) => write!(f, "{}", v),
            Self::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Number(f64);

impl std::str::FromStr for Number {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == self.0.trunc() {
            // No integer-like formatting for floats.
            write!(f, "{}.0", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
