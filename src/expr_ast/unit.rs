use std::fmt;

use crate::{
    lex::{Token, tt},
    literal::Literal,
};

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl From<Binary> for Expr {
    fn from(binary: Binary) -> Self {
        Self::Binary(binary)
    }
}

impl From<Grouping> for Expr {
    fn from(grouping: Grouping) -> Self {
        Self::Grouping(grouping)
    }
}

impl From<Literal> for Expr {
    fn from(literal: Literal) -> Self {
        Self::Literal(literal)
    }
}

impl From<Unary> for Expr {
    fn from(unary: Unary) -> Self {
        Self::Unary(unary)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binary(v) => write!(f, "{}", v),
            Self::Grouping(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
            Self::Unary(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub op: BinaryOp,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn bind(&mut self, op: BinaryOp, right: Expr) {
        let right = Box::new(right);
        let cloned = self.clone();

        *self = if self.op.priority() < op.priority() {
            // If given operator has higher priority, bind it to its right.
            Binary {
                left: cloned.left,
                op: self.op,
                right: Box::new(Expr::Binary(Binary {
                    left: self.clone().right,
                    op,
                    right,
                })),
            }
        } else {
            // Otherwise, bind it to the left.
            Binary {
                left: Box::new(cloned.into()),
                op,
                right,
            }
        }
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.op, self.left, self.right)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Plus,
    Minus,
    Star,
    Slash,
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
}

impl BinaryOp {
    pub fn from_token(token: &Token) -> Option<Self> {
        match token.token_type {
            tt!("+") => Some(Self::Plus),
            tt!("-") => Some(Self::Minus),
            tt!("*") => Some(Self::Star),
            tt!("/") => Some(Self::Slash),
            tt!("==") => Some(Self::EqualEqual),
            tt!("!=") => Some(Self::BangEqual),
            tt!(">") => Some(Self::Greater),
            tt!(">=") => Some(Self::GreaterEqual),
            tt!("<") => Some(Self::Less),
            tt!("<=") => Some(Self::LessEqual),
            tt!("and") => Some(Self::And),
            tt!("or") => Some(Self::Or),
            _ => None,
        }
    }

    pub fn priority(&self) -> u8 {
        match self {
            Self::Plus | Self::Minus => 3,
            Self::Star | Self::Slash => 4,
            Self::EqualEqual
            | Self::BangEqual
            | Self::Greater
            | Self::GreaterEqual
            | Self::Less
            | Self::LessEqual => 2,
            Self::And => 1,
            Self::Or => 0,
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Star => "*",
            Self::Slash => "/",
            Self::EqualEqual => "==",
            Self::BangEqual => "!=",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::And => "and",
            Self::Or => "or",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub inner: Box<Expr>,
}

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(group {})", self.inner)
    }
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub op: UnaryOp,
    pub right: Box<Expr>,
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.op, self.right)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Bang,
    Minus,
}

impl UnaryOp {
    pub fn from_token(token: &Token) -> Option<Self> {
        match token.token_type {
            tt!("!") => Some(Self::Bang),
            tt!("-") => Some(Self::Minus),
            _ => None,
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Bang => "!",
            Self::Minus => "-",
        };
        write!(f, "{}", s)
    }
}
