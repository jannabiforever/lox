use std::{cmp, fmt, ops};

use crate::{
    // function::LoxFunction,
    function::{LoxFunction, RustFunction},
    mac::impl_from,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Literal {
    Boolean(bool),
    #[default]
    Nil,
    Number(Number),
    String(String),
}

impl Literal {
    /// Check if the literal is truthy.
    /// In Lox, nil and false are falsy, everything else is truthy.
    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::Nil => false,
            Literal::Boolean(b) => *b,
            _ => true,
        }
    }

    /// When the current command is `evaluate` or `run`, pretty-print the
    /// literal.
    pub fn pretty(&self) -> String {
        match self {
            Literal::Number(Number(n)) => n.to_string(),
            v => v.to_string(),
        }
    }

    pub fn number_or<E>(self, error: E) -> Result<Number, E> {
        match self {
            Self::Number(v) => Ok(v),
            _ => Err(error),
        }
    }

    pub fn string_or<E>(self, error: E) -> Result<String, E> {
        match self {
            Self::String(v) => Ok(v),
            _ => Err(error),
        }
    }
}

impl_from!(Literal: Number, String);

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        Literal::Boolean(b)
    }
}

/// When the current command is not `evaluate` or `run`, format as following:
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

/// In Lox, a number is always a 64-bit floating point number.
/// This is a wrapper around f64 to implement the Display trait as needed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Number(pub f64);

impl std::str::FromStr for Number {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl fmt::Display for Number {
    /// In Lox, a number should be always displayed with a decimal point.
    /// If the number is an integer, it should be displayed with a ".0" suffix.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == self.0.trunc() {
            write!(f, "{}.0", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl ops::Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ops::Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl ops::Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl ops::Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl cmp::PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum LoxValue<'src> {
    Literal(Literal),
    RustFunction(RustFunction<'src>),
    LoxFunction(LoxFunction<'src>),
}

impl From<Literal> for LoxValue<'_> {
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}

impl_from!('a LoxValue: LoxFunction, RustFunction);

impl LoxValue<'_> {
    pub fn is_literal_and<F: Fn(&Literal) -> bool>(&self, f: F) -> bool {
        if let Self::Literal(l) = self {
            f(l)
        } else {
            false
        }
    }

    pub fn literal_or<E>(self, error: E) -> Result<Literal, E> {
        match self {
            Self::Literal(l) => Ok(l),
            _ => Err(error),
        }
    }
}

impl Default for LoxValue<'_> {
    fn default() -> Self {
        LoxValue::Literal(Literal::default())
    }
}

/// Note: Displaying a literal as a lox value is only possible when running
/// `evaluate` or `run` commands, so literal need to be pretty-printed.
impl fmt::Display for LoxValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // trim ".0" when treating as lox value.
            Self::Literal(l) => write!(f, "{}", l.pretty()),
            // Self::LoxFunction(lf) => write!(f, "{lf}"),
            Self::RustFunction(rf) => write!(f, "{rf}"),
            Self::LoxFunction(lf) => write!(f, "{lf}"),
        }
    }
}
