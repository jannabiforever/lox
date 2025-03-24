use std::fmt;

use crate::lex::{Token, TokenType};

#[derive(Debug, Clone)]
pub(crate) struct OwnedToken {
    pub source: String,
    pub token_type: TokenType,
}

impl OwnedToken {
    pub fn to_token(&self) -> Token {
        Token {
            source: &self.source,
            token_type: self.token_type,
        }
    }
}

impl fmt::Display for OwnedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_token())
    }
}
