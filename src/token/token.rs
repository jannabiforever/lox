use std::fmt;

use crate::{literal::Number, mac::tt};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Token<'a> {
    /// The reference to the actual source.
    /// Be careful with [`TokenType::String`], because it contains the quotes.
    /// e.g) \"Hello, World!\"
    /// It is used for translating literal tokens to its own value.
    pub line: usize,
    pub src: &'a str,
    pub token_type: TokenType,
}

impl<'a> Token<'a> {
    /// Create reserved word tokens or identifiers.
    pub(crate) fn word(line: usize, src: &'a str) -> Self {
        match src {
            "and" => Token {
                line,
                src,
                token_type: tt!("and"),
            },
            "class" => Token {
                line,
                src,
                token_type: tt!("class"),
            },
            "else" => Token {
                line,
                src,
                token_type: tt!("else"),
            },
            "false" => Token {
                line,
                src,
                token_type: tt!("false"),
            },
            "fun" => Token {
                line,
                src,
                token_type: tt!("fun"),
            },
            "for" => Token {
                line,
                src,
                token_type: tt!("for"),
            },
            "if" => Token {
                line,
                src,
                token_type: tt!("if"),
            },
            "nil" => Token {
                line,
                src,
                token_type: tt!("nil"),
            },
            "or" => Token {
                line,
                src,
                token_type: tt!("or"),
            },
            "print" => Token {
                line,
                src,
                token_type: tt!("print"),
            },
            "return" => Token {
                line,
                src,
                token_type: tt!("return"),
            },
            "super" => Token {
                line,
                src,
                token_type: tt!("super"),
            },
            "this" => Token {
                line,
                src,
                token_type: tt!("this"),
            },
            "true" => Token {
                line,
                src,
                token_type: tt!("true"),
            },
            "var" => Token {
                line,
                src,
                token_type: tt!("var"),
            },
            "while" => Token {
                line,
                src,
                token_type: tt!("while"),
            },
            _ => Token {
                line,
                src,
                token_type: tt!("identifier"),
            },
        }
    }

    pub(crate) fn number(line: usize, src: &'a str) -> Self {
        Token {
            line,
            src,
            token_type: tt!("number"),
        }
    }

    pub(crate) fn string(line: usize, src: &'a str) -> Self {
        Token {
            line,
            src,
            token_type: tt!("string"),
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.token_type,
            self.src,
            match &self.token_type {
                TokenType::String => self.src.trim_matches('"').to_string(),
                TokenType::Number => self
                    .src
                    .parse::<Number>()
                    .expect("Malformed number.") // Should be unreachable.
                    .to_string(),
                _ => "null".to_string(),
            }
        )
    }
}

#[derive(PartialEq, Clone, Copy)]
pub(crate) enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

impl fmt::Debug for TokenType {
    /// UPPER_SNAKE_CASE
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LeftParen => write!(f, "LEFT_PAREN"),
            Self::RightParen => write!(f, "RIGHT_PAREN"),
            Self::LeftBrace => write!(f, "LEFT_BRACE"),
            Self::RightBrace => write!(f, "RIGHT_BRACE"),
            Self::Comma => write!(f, "COMMA"),
            Self::Dot => write!(f, "DOT"),
            Self::Minus => write!(f, "MINUS"),
            Self::Plus => write!(f, "PLUS"),
            Self::Semicolon => write!(f, "SEMICOLON"),
            Self::Slash => write!(f, "SLASH"),
            Self::Star => write!(f, "STAR"),
            Self::Bang => write!(f, "BANG"),
            Self::BangEqual => write!(f, "BANG_EQUAL"),
            Self::Equal => write!(f, "EQUAL"),
            Self::EqualEqual => write!(f, "EQUAL_EQUAL"),
            Self::Greater => write!(f, "GREATER"),
            Self::GreaterEqual => write!(f, "GREATER_EQUAL"),
            Self::Less => write!(f, "LESS"),
            Self::LessEqual => write!(f, "LESS_EQUAL"),
            Self::Identifier => write!(f, "IDENTIFIER"),
            Self::String => write!(f, "STRING"),
            Self::Number => write!(f, "NUMBER"),
            Self::And => write!(f, "AND"),
            Self::Class => write!(f, "CLASS"),
            Self::Else => write!(f, "ELSE"),
            Self::False => write!(f, "FALSE"),
            Self::Fun => write!(f, "FUN"),
            Self::For => write!(f, "FOR"),
            Self::If => write!(f, "IF"),
            Self::Nil => write!(f, "NIL"),
            Self::Or => write!(f, "OR"),
            Self::Print => write!(f, "PRINT"),
            Self::Return => write!(f, "RETURN"),
            Self::Super => write!(f, "SUPER"),
            Self::This => write!(f, "THIS"),
            Self::True => write!(f, "TRUE"),
            Self::Var => write!(f, "VAR"),
            Self::While => write!(f, "WHILE"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}
