use crate::literal::Number;
use std::fmt;

use super::tt;

#[derive(Debug)]
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
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::Dot => write!(f, "DOT"),
            TokenType::Minus => write!(f, "MINUS"),
            TokenType::Plus => write!(f, "PLUS"),
            TokenType::Semicolon => write!(f, "SEMICOLON"),
            TokenType::Slash => write!(f, "SLASH"),
            TokenType::Star => write!(f, "STAR"),
            TokenType::Bang => write!(f, "BANG"),
            TokenType::BangEqual => write!(f, "BANG_EQUAL"),
            TokenType::Equal => write!(f, "EQUAL"),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenType::Greater => write!(f, "GREATER"),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TokenType::Less => write!(f, "LESS"),
            TokenType::LessEqual => write!(f, "LESS_EQUAL"),
            TokenType::Identifier => write!(f, "IDENTIFIER"),
            TokenType::String => write!(f, "STRING"),
            TokenType::Number => write!(f, "NUMBER"),
            TokenType::And => write!(f, "AND"),
            TokenType::Class => write!(f, "CLASS"),
            TokenType::Else => write!(f, "ELSE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::Fun => write!(f, "FUN"),
            TokenType::For => write!(f, "FOR"),
            TokenType::If => write!(f, "IF"),
            TokenType::Nil => write!(f, "NIL"),
            TokenType::Or => write!(f, "OR"),
            TokenType::Print => write!(f, "PRINT"),
            TokenType::Return => write!(f, "RETURN"),
            TokenType::Super => write!(f, "SUPER"),
            TokenType::This => write!(f, "THIS"),
            TokenType::True => write!(f, "TRUE"),
            TokenType::Var => write!(f, "VAR"),
            TokenType::While => write!(f, "WHILE"),
            TokenType::Eof => write!(f, "EOF"),
        }
    }
}
