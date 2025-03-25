use super::{
    regex::{
        COMMENT_REGEX, NUMBER_REGEX, RAW_STRING_REGEX, UNTERMINATED_STRING_REGEX, WHITESPACE_REGEX,
        WORD_REGEX,
    },
    tt,
};
use crate::{
    error::{ErrorReporter, LexError, LoxError, WithLine},
    lex::Token,
};
use regex::Regex;

type TokenResult<'a> = Result<Token<'a>, LexError>;
/// Wrapped token result with line number, which is only used for public call from main.rs.
type TokenResultWithLine<'a> = WithLine<Result<Token<'a>, LoxError>>;

/// Public function to tokenize the source code.
pub fn scan(source: &str) -> Vec<WithLine<Result<Token, LoxError>>> {
    let mut lexer = Lexer::new(source);
    lexer.scan_tokens().into_iter().collect()
}

#[derive(Clone)]
pub(crate) struct Lexer<'a> {
    stream: CharStream<'a>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Self {
            stream: CharStream::new(source),
        }
    }

    fn scan_tokens(&mut self) -> Vec<TokenResultWithLine<'a>> {
        let mut tokens = Vec::new();
        loop {
            let token = self.scan_token();
            match token {
                Ok(token) => {
                    let is_eof = token.token_type == tt!("");
                    tokens.push(self.get_lox_ok(token));

                    if is_eof {
                        break;
                    }
                }
                Err(err) => {
                    tokens.push(self.get_lox_err(err));
                }
            }
        }
        tokens
    }

    pub(crate) fn scan_token(&mut self) -> TokenResult<'a> {
        let token: Token<'a> = if self.stream.pop_match(&WHITESPACE_REGEX).is_some() {
            // Skip whitespace.
            return self.scan_token();
        } else if self.stream.pop_match(&COMMENT_REGEX).is_some() {
            // Skip this line.
            return self.scan_token();
        } else if let Some(source) = self.stream.pop_match(&NUMBER_REGEX) {
            // Get Number token.
            Token {
                source,
                token_type: tt!("number"),
            }
        } else if let Some(source) = self.stream.pop_match(&RAW_STRING_REGEX) {
            // Get String token.
            Token {
                source,
                token_type: tt!("string"),
            }
        } else if self.stream.pop_match(&UNTERMINATED_STRING_REGEX).is_some() {
            // Unterminated string.
            return Err(LexError::UnterminatedString);
        } else if let Some(source) = self.stream.pop_match(&WORD_REGEX) {
            // Get reserved word or identifier token.
            match source {
                "and" => Token {
                    source,
                    token_type: tt!("and"),
                },
                "class" => Token {
                    source,
                    token_type: tt!("class"),
                },
                "else" => Token {
                    source,
                    token_type: tt!("else"),
                },
                "false" => Token {
                    source,
                    token_type: tt!("false"),
                },
                "fun" => Token {
                    source,
                    token_type: tt!("fun"),
                },
                "for" => Token {
                    source,
                    token_type: tt!("for"),
                },
                "if" => Token {
                    source,
                    token_type: tt!("if"),
                },
                "nil" => Token {
                    source,
                    token_type: tt!("nil"),
                },
                "or" => Token {
                    source,
                    token_type: tt!("or"),
                },
                "print" => Token {
                    source,
                    token_type: tt!("print"),
                },
                "return" => Token {
                    source,
                    token_type: tt!("return"),
                },
                "super" => Token {
                    source,
                    token_type: tt!("super"),
                },
                "this" => Token {
                    source,
                    token_type: tt!("this"),
                },
                "true" => Token {
                    source,
                    token_type: tt!("true"),
                },
                "var" => Token {
                    source,
                    token_type: tt!("var"),
                },
                "while" => Token {
                    source,
                    token_type: tt!("while"),
                },
                _ => Token {
                    source,
                    token_type: tt!("identifier"),
                },
            }
        } else {
            // else current token is a single or double character token.
            if let Some(ch) = self.stream.advance() {
                match ch {
                    '(' => Token {
                        source: "(",
                        token_type: tt!("("),
                    },
                    ')' => Token {
                        source: ")",
                        token_type: tt!(")"),
                    },
                    '{' => Token {
                        source: "{",
                        token_type: tt!("{"),
                    },
                    '}' => Token {
                        source: "}",
                        token_type: tt!("}"),
                    },
                    ',' => Token {
                        source: ",",
                        token_type: tt!(","),
                    },
                    '.' => Token {
                        source: ".",
                        token_type: tt!("."),
                    },
                    '-' => Token {
                        source: "-",
                        token_type: tt!("-"),
                    },
                    '+' => Token {
                        source: "+",
                        token_type: tt!("+"),
                    },
                    ';' => Token {
                        source: ";",
                        token_type: tt!(";"),
                    },
                    '*' => Token {
                        source: "*",
                        token_type: tt!("*"),
                    },
                    '!' => {
                        if self.stream.peek() == Some('=') {
                            self.stream.advance();
                            Token {
                                source: "!=",
                                token_type: tt!("!="),
                            }
                        } else {
                            Token {
                                source: "!",
                                token_type: tt!("!"),
                            }
                        }
                    }
                    '=' => {
                        if self.stream.peek() == Some('=') {
                            self.stream.advance();
                            Token {
                                source: "==",
                                token_type: tt!("=="),
                            }
                        } else {
                            Token {
                                source: "=",
                                token_type: tt!("="),
                            }
                        }
                    }
                    '>' => {
                        if self.stream.peek() == Some('=') {
                            self.stream.advance();
                            Token {
                                source: ">=",
                                token_type: tt!(">="),
                            }
                        } else {
                            Token {
                                source: ">",
                                token_type: tt!(">"),
                            }
                        }
                    }
                    '<' => {
                        if self.stream.peek() == Some('=') {
                            self.stream.advance();
                            Token {
                                source: "<=",
                                token_type: tt!("<="),
                            }
                        } else {
                            Token {
                                source: "<",
                                token_type: tt!("<"),
                            }
                        }
                    }
                    '/' => Token {
                        source: "/",
                        token_type: tt!("/"),
                    },
                    _ => {
                        return Err(LexError::UnexpectedChar(ch));
                    }
                }
            } else {
                Token::eof()
            }
        };

        Ok(token)
    }
}

impl ErrorReporter<LexError> for Lexer<'_> {
    fn line(&self) -> usize {
        self.stream.line
    }
}

/// The stream that is responsible for character handling.
#[derive(Clone)]
struct CharStream<'a> {
    source: &'a str,
    /// The index of next character to be read.
    pos: usize,
    /// The current line number.
    line: usize,
}

impl<'a> CharStream<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
            line: 1,
        }
    }

    /// Get the remaining source from the current position.
    fn remainee(&self) -> &'a str {
        &self.source[self.pos..]
    }

    /// Pop the matched string from the remaining source.
    /// Each regex should start with `^` to match the start of the string.
    fn pop_match(&mut self, regex: &Regex) -> Option<&'a str> {
        regex.find(self.remainee()).map(|mat| {
            let matched = mat.as_str();
            // Update line number.
            if matched.contains('\n') {
                self.line += matched.lines().count() - 1;
            }
            self.pos += matched.len();
            matched
        })
    }

    /// Advance the position by one character.
    fn advance(&mut self) -> Option<char> {
        let next_char = self.remainee().chars().next();
        if let Some(&next_char) = next_char.as_ref() {
            if next_char == '\n' {
                self.line += 1;
            }
            self.pos += next_char.len_utf8();
            Some(next_char)
        } else {
            None
        }
    }

    /// Peek the next character.
    fn peek(&self) -> Option<char> {
        self.remainee().chars().next()
    }
}
