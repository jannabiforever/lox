use regex::Regex;

use crate::error::LoxError;

use super::{
    regex::{COMMENT_REGEX, NUMBER_REGEX, RAW_STRING_REGEX, WHITESPACE_REGEX, WORD_REGEX},
    token::Token,
    tt,
    TokenizeError::{self, *},
};

pub(crate) struct Tokenizer<'a> {
    src: &'a str,
    pos: usize,
    line: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            pos: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Result<Token<'a>, LoxError>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            let is_eof = token
                .as_ref()
                .is_ok_and(|token| token.token_type == tt!(""));
            tokens.push(token.map_err(|err| LoxError {
                line: self.line,
                kind: err.into(),
            }));
            if is_eof {
                break;
            }
        }

        tokens
    }

    fn next_token(&mut self) -> Result<Token<'a>, TokenizeError> {
        let token = if let Some(_) = self.consume_match(&*COMMENT_REGEX) {
            // If we find a comment, we skip it and continue to the next token.
            self.next_token()?
        } else if let Some(_) = self.consume_match(&*WHITESPACE_REGEX) {
            self.next_token()?
        } else if let Some(ch) = self.advance() {
            match ch {
                '(' => Token {
                    src: "(",
                    token_type: tt!("("),
                },
                ')' => Token {
                    src: ")",
                    token_type: tt!(")"),
                },
                '{' => Token {
                    src: "{",
                    token_type: tt!("{"),
                },
                '}' => Token {
                    src: "}",
                    token_type: tt!("}"),
                },
                ',' => Token {
                    src: ",",
                    token_type: tt!(","),
                },
                '.' => Token {
                    src: ".",
                    token_type: tt!("."),
                },
                '-' => Token {
                    src: "-",
                    token_type: tt!("-"),
                },
                '+' => Token {
                    src: "+",
                    token_type: tt!("+"),
                },
                ';' => Token {
                    src: ";",
                    token_type: tt!(";"),
                },
                '*' => Token {
                    src: "*",
                    token_type: tt!("*"),
                },
                '=' => {
                    if self.remain().starts_with('=') {
                        self.advance();
                        Token {
                            src: "==",
                            token_type: tt!("=="),
                        }
                    } else {
                        Token {
                            src: "=",
                            token_type: tt!("="),
                        }
                    }
                }
                '!' => {
                    if self.remain().starts_with('=') {
                        self.advance();
                        Token {
                            src: "!=",
                            token_type: tt!("!="),
                        }
                    } else {
                        Token {
                            src: "!",
                            token_type: tt!("!"),
                        }
                    }
                }
                '>' => {
                    if self.remain().starts_with('=') {
                        self.advance();
                        Token {
                            src: ">=",
                            token_type: tt!(">="),
                        }
                    } else {
                        Token {
                            src: ">",
                            token_type: tt!(">"),
                        }
                    }
                }
                '<' => {
                    if self.remain().starts_with('=') {
                        self.advance();
                        Token {
                            src: "<=",
                            token_type: tt!("<="),
                        }
                    } else {
                        Token {
                            src: "<",
                            token_type: tt!("<"),
                        }
                    }
                }
                '/' => Token {
                    src: "/",
                    token_type: tt!("/"),
                },
                ch => return Err(UnexpectedCharacter(ch)),
            }
        } else {
            Token::eof()
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

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

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

    fn is_at_end(&self) -> bool {
        self.pos >= self.src.len()
    }
}
