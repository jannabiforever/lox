use super::regex::{
    COMMENT_REGEX, NUMBER_REGEX, RAW_STRING_REGEX, UNTERMINATED_STRING_REGEX, WHITESPACE_REGEX,
    WORD_REGEX,
};
use crate::{
    error::{ErrorReporter, LexError, WithLine},
    lex::{Token, TokenType},
};
use regex::Regex;

type TokenResult<'a> = Result<Token<'a>, LexError>;
type TokenResultWithLine<'a> = WithLine<Result<Token<'a>, LexError>>;

pub fn scan<'a>(source: &'a str) -> Vec<TokenResultWithLine<'a>> {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens()
}

pub struct Scanner<'a> {
    stream: CharStream<'a>,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
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
                    let is_eof = token.token_type == TokenType::Eof;
                    tokens.push(self.get_ok(token));

                    if is_eof {
                        break;
                    }
                }
                Err(err) => {
                    tokens.push(self.get_err(err));
                }
            }
        }
        tokens
    }

    fn scan_token(&mut self) -> TokenResult<'a> {
        let token: Token<'a> = if let Some(_) = self.stream.pop_match(&WHITESPACE_REGEX) {
            // Skip whitespace.
            return self.scan_token();
        } else if let Some(_) = self.stream.pop_match(&COMMENT_REGEX) {
            // Skip this line.
            return self.scan_token();
        } else if let Some(source) = self.stream.pop_match(&NUMBER_REGEX) {
            // Get Number token.
            Token {
                source,
                token_type: TokenType::Number,
            }
        } else if let Some(source) = self.stream.pop_match(&RAW_STRING_REGEX) {
            // Get String token.
            Token {
                source,
                token_type: TokenType::String,
            }
        } else if let Some(_) = self.stream.pop_match(&UNTERMINATED_STRING_REGEX) {
            // Unterminated string.
            return Err(LexError::UnterminatedString);
        } else if let Some(source) = self.stream.pop_match(&WORD_REGEX) {
            // Get reserved word or identifier token.
            match source {
                "and" => Token {
                    source,
                    token_type: TokenType::And,
                },
                "class" => Token {
                    source,
                    token_type: TokenType::Class,
                },
                "else" => Token {
                    source,
                    token_type: TokenType::Else,
                },
                "false" => Token {
                    source,
                    token_type: TokenType::False,
                },
                "fun" => Token {
                    source,
                    token_type: TokenType::Fun,
                },
                "for" => Token {
                    source,
                    token_type: TokenType::For,
                },
                "if" => Token {
                    source,
                    token_type: TokenType::If,
                },
                "nil" => Token {
                    source,
                    token_type: TokenType::Nil,
                },
                "or" => Token {
                    source,
                    token_type: TokenType::Or,
                },
                "print" => Token {
                    source,
                    token_type: TokenType::Print,
                },
                "return" => Token {
                    source,
                    token_type: TokenType::Return,
                },
                "super" => Token {
                    source,
                    token_type: TokenType::Super,
                },
                "this" => Token {
                    source,
                    token_type: TokenType::This,
                },
                "true" => Token {
                    source,
                    token_type: TokenType::True,
                },
                "var" => Token {
                    source,
                    token_type: TokenType::Var,
                },
                "while" => Token {
                    source,
                    token_type: TokenType::While,
                },
                _ => Token {
                    source,
                    token_type: TokenType::Identifier,
                },
            }
        } else {
            // else current token is a single or double character token.
            if let Some(ch) = self.stream.advance() {
                match ch {
                    '(' => Token {
                        source: "(",
                        token_type: TokenType::LeftParen,
                    },
                    ')' => Token {
                        source: ")",
                        token_type: TokenType::RightParen,
                    },
                    '{' => Token {
                        source: "{",
                        token_type: TokenType::LeftBrace,
                    },
                    '}' => Token {
                        source: "}",
                        token_type: TokenType::RightBrace,
                    },
                    ',' => Token {
                        source: ",",
                        token_type: TokenType::Comma,
                    },
                    '.' => Token {
                        source: ".",
                        token_type: TokenType::Dot,
                    },
                    '-' => Token {
                        source: "-",
                        token_type: TokenType::Minus,
                    },
                    '+' => Token {
                        source: "+",
                        token_type: TokenType::Plus,
                    },
                    ';' => Token {
                        source: ";",
                        token_type: TokenType::Semicolon,
                    },
                    '*' => Token {
                        source: "*",
                        token_type: TokenType::Star,
                    },
                    '!' => {
                        if self.stream.peek() == Some('=') {
                            self.stream.advance();
                            Token {
                                source: "!=",
                                token_type: TokenType::BangEqual,
                            }
                        } else {
                            Token {
                                source: "!",
                                token_type: TokenType::Bang,
                            }
                        }
                    }
                    '=' => {
                        if self.stream.peek() == Some('=') {
                            self.stream.advance();
                            Token {
                                source: "==",
                                token_type: TokenType::EqualEqual,
                            }
                        } else {
                            Token {
                                source: "=",
                                token_type: TokenType::Equal,
                            }
                        }
                    }
                    '>' => {
                        if self.stream.peek() == Some('=') {
                            self.stream.advance();
                            Token {
                                source: ">=",
                                token_type: TokenType::GreaterEqual,
                            }
                        } else {
                            Token {
                                source: ">",
                                token_type: TokenType::Greater,
                            }
                        }
                    }
                    '<' => {
                        if self.stream.peek() == Some('=') {
                            self.stream.advance();
                            Token {
                                source: "<=",
                                token_type: TokenType::LessEqual,
                            }
                        } else {
                            Token {
                                source: "<",
                                token_type: TokenType::Less,
                            }
                        }
                    }
                    '/' => Token {
                        source: "/",
                        token_type: TokenType::Slash,
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

impl ErrorReporter<LexError> for Scanner<'_> {
    fn line(&self) -> usize {
        self.stream.line
    }
}

/// The stream that is responsible for character handling.
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
