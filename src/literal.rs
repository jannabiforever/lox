use std::{cell::RefCell, cmp, fmt, io::Write, ops, rc::Rc};

use crate::{
    env::{Env, Evaluatable, EvaluateError},
    function::RustFunction,
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

    pub fn pretty(&self) -> String {
        match self {
            Literal::Number(Number(n)) => n.to_string(),
            v => v.to_string(),
        }
    }
}

impl Evaluatable for Literal {
    fn eval<W: Write>(&self, _: Rc<RefCell<Env<W>>>) -> Result<LoxValue, EvaluateError> {
        Ok(self.clone().into())
    }
}

impl_from!(Literal: Number, String);

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        Literal::Boolean(b)
    }
}

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
pub(crate) enum LoxValue {
    Literal(Literal),
    RustFunction(RustFunction),
    // LoxFunction(LoxFunction),
}

impl_from!(LoxValue: Literal, RustFunction);

impl LoxValue {
    pub fn pretty(&self) -> String {
        match self {
            Self::Literal(literal) => literal.pretty(),
            _ => todo!("Implement pretty print."),
        }
    }

    pub fn is_literal_and<F: Fn(&Literal) -> bool>(&self, f: F) -> bool {
        if let Self::Literal(l) = self {
            f(l)
        } else {
            false
        }
    }
}
