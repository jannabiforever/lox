use regex::Regex;

use super::regex::COMMENT_REGEX;
use super::regex::NUMBER_REGEX;
use super::regex::RAW_STRING_REGEX;
use super::regex::UNTERMINATED_STRING_REGEX;
use super::regex::WHITESPACE_REGEX;
use super::regex::WORD_REGEX;
use super::token::Token;
use super::TokenType;
use super::TokenizeError::*;
use super::TokenizeError::{self};
use crate::error::IntoLoxError;
use crate::error::LoxError;
use crate::mac::tt;

pub(crate) struct Tokenizer<'a> {
    src: &'a str,
    pos: usize,
    line: usize,
}

impl<'a> Tokenizer<'a> {
    pub(crate) fn new(src: &'a str) -> Self {
        Self {
            src,
            pos: 0,
            line: 1,
        }
    }

    /// Wrap the result of `next_token` with the current line number,
    /// and collect them until eof is returned.
    pub(crate) fn tokenize(&mut self) -> Vec<Result<Token<'a>, LoxError<TokenizeError>>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token_with_line();
            match token.as_ref() {
                Ok(Token {
                    token_type: tt!(""),
                    ..
                }) => {
                    tokens.push(token);
                    break;
                }
                _ => tokens.push(token),
            }
        }

        tokens
    }

    fn next_token_with_line(&mut self) -> Result<Token<'a>, LoxError<TokenizeError>> {
        self.next_token().map_err(|e| e.error(self.line))
    }

    fn next_token(&mut self) -> Result<Token<'a>, TokenizeError> {
        let token = if self.consume_match(&COMMENT_REGEX).is_some() {
            // If we find a comment, we skip it and continue to the next token.
            self.next_token()?
        } else if self.consume_match(&WHITESPACE_REGEX).is_some() {
            // If we find a whitespace, we skip it and continue to the next token.
            self.next_token()?
        } else if let Some(src) = self.consume_match(&RAW_STRING_REGEX) {
            Token::string(self.line, src)
        } else if self.consume_match(&UNTERMINATED_STRING_REGEX).is_some() {
            // It is confirmed that the string doesn't have a closing quote,
            // which is not determined by [`UNTERMINATED_STRING_REGEX`], but by
            // [`RAW_STRING_REGEX`] above.
            return Err(UnterminatedString);
        } else if let Some(src) = self.consume_match(&NUMBER_REGEX) {
            Token::number(self.line, src)
        } else if let Some(src) = self.consume_match(&WORD_REGEX) {
            Token::word(self.line, src)
        } else if let Some(ch) = self.advance() {
            match ch {
                '(' => self.token("(", tt!("(")),
                ')' => self.token(")", tt!(")")),
                '{' => self.token("{", tt!("{")),
                '}' => self.token("}", tt!("}")),
                ',' => self.token(",", tt!(",")),
                '.' => self.token(".", tt!(".")),
                '-' => self.token("-", tt!("-")),
                '+' => self.token("+", tt!("+")),
                ';' => self.token(";", tt!(";")),
                '*' => self.token("*", tt!("*")),
                '=' => {
                    if self.remain().starts_with('=') {
                        self.advance();
                        self.token("==", tt!("=="))
                    } else {
                        self.token("=", tt!("="))
                    }
                }
                '!' => {
                    if self.remain().starts_with('=') {
                        self.advance();
                        self.token("!=", tt!("!="))
                    } else {
                        self.token("!", tt!("!"))
                    }
                }
                '>' => {
                    if self.remain().starts_with('=') {
                        self.advance();
                        self.token(">=", tt!(">="))
                    } else {
                        self.token(">", tt!(">"))
                    }
                }
                '<' => {
                    if self.remain().starts_with('=') {
                        self.advance();
                        self.token("<=", tt!("<="))
                    } else {
                        self.token("<", tt!("<"))
                    }
                }
                '/' => self.token("/", tt!("/")),
                ch => return Err(UnexpectedCharacter(ch)),
            }
        } else {
            // None of the above, so we must be at the end of the file.
            self.token("", tt!(""))
        };

        Ok(token)
    }

    /// Try match the regex from the current position in the src,
    /// and consume the match if it exists.
    fn consume_match(&mut self, regex: &Regex) -> Option<&'a str> {
        let found = regex.find(self.remain()).map(|m| m.as_str())?;
        self.pos += found.len();
        self.line += found.chars().filter(|&c| c == '\n').count();
        Some(found)
    }

    /// Consume a character from the src and return it.
    fn advance(&mut self) -> Option<char> {
        let c = self.src[self.pos..].chars().next()?;
        self.pos += c.len_utf8();
        if c == '\n' {
            self.line += 1;
        }
        Some(c)
    }

    fn remain(&self) -> &'a str {
        &self.src[self.pos..]
    }

    fn token(&self, src: &'a str, token_type: TokenType) -> Token<'a> {
        Token {
            line: self.line,
            src,
            token_type,
        }
    }
}
