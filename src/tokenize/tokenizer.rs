use regex::Regex;

use super::{
    regex::{NUMBER_REGEX, RAW_STRING_REGEX, WHITESPACE_REGEX, WORD_REGEX},
    token::Token,
    tt,
};

pub(crate) struct Tokenizer<'a> {
    source: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, pos: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();

        loop {
            match self.peek() {
                // Some(whitespace_ch) if whitespace_ch.is_ascii_whitespace() => {
                //     // Consume whitespace, and just don't use it.
                //     self.consume_match(&*WHITESPACE_REGEX);
                // }
                // Some('0'..='9') => {
                //     // Consume a number.
                //     let src = self.consume_match(&*NUMBER_REGEX);
                //     tokens.push(Token {
                //         src,
                //         token_type: tt!("number"),
                //     });
                // }
                // Some('"') => {
                //     // Consume a raw string.
                //     let src = self.consume_match(&*RAW_STRING_REGEX);
                //     tokens.push(Token {
                //         src,
                //         token_type: tt!("string"),
                //     });
                // }
                Some('(') => {
                    // Consume a left parenthesis.
                    self.pos += '('.len_utf8();
                    tokens.push(Token {
                        src: "(",
                        token_type: tt!("("),
                    });
                }
                Some(')') => {
                    // Consume a right parenthesis.
                    self.pos += ')'.len_utf8();
                    tokens.push(Token {
                        src: ")",
                        token_type: tt!(")"),
                    });
                }
                Some('{') => {
                    // Consume a left brace.
                    self.pos += '{'.len_utf8();
                    tokens.push(Token {
                        src: "{",
                        token_type: tt!("{"),
                    });
                }
                Some('}') => {
                    // Consume a right brace.
                    self.pos += '}'.len_utf8();
                    tokens.push(Token {
                        src: "}",
                        token_type: tt!("}"),
                    });
                }
                None => {
                    tokens.push(Token::eof());
                    break;
                }
                _ => todo!("implement more token types"),
            }
        }

        tokens
    }

    /// Try match the regex from the current position in the source,
    /// and consume the match if it exists.
    fn consume_match(&mut self, regex: &Regex) -> &'a str {
        let found = regex.find(self.remain()).map(|m| m.as_str()).unwrap();
        self.pos += found.len();
        found
    }

    fn peek(&self) -> Option<char> {
        self.remain().chars().next()
    }

    fn remain(&self) -> &'a str {
        &self.source[self.pos..]
    }
}
