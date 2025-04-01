use super::Token;

pub struct TokenStream<'a> {
    tokens: &'a [Token<'a>],
    current: usize,
    eof: Option<&'a Token<'a>>,
    line: usize,
}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Self {
            tokens,
            current: 0,
            eof: None,
            line: 0,
        }
    }

    pub fn next(&mut self) -> &'a Token<'a> {
        self.assert_if_expired_then_eof();

        if self.expired() {
            self.eof.as_ref().unwrap()
        } else {
            let token = &self.tokens[self.current];
            self.line = token.line;
            self.current += 1;
            if self.expired() {
                self.eof = Some(token);
            }
            token
        }
    }

    pub fn peek(&self) -> &'a Token<'a> {
        self.assert_if_expired_then_eof();

        if self.expired() {
            self.eof.as_ref().unwrap()
        } else {
            &self.tokens[self.current]
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    fn expired(&self) -> bool {
        self.current >= self.tokens.len()
    }

    // Note: Could be done with XOR, but this is more readable.
    fn assert_if_expired_then_eof(&self) {
        if self.current >= self.tokens.len() {
            assert!(
                self.eof.is_some(),
                "TokenStream has expired, but eof is None"
            );
        } else {
            assert!(
                self.eof.is_none(),
                "TokenStream has not expired, but eof is Some"
            )
        }
    }
}
