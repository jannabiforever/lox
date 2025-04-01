use super::Token;

enum TokenStreamState<'a> {
    /// Index of the next token to be returned.
    NotExpired(usize),
    /// The stream has expired, and the last token(eof) is stored here.
    Expired(&'a Token<'a>),
}

pub struct TokenStream<'a> {
    tokens: &'a [Token<'a>],
    line: usize,
    state: TokenStreamState<'a>,
}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Self {
            tokens,
            state: TokenStreamState::NotExpired(0),
            line: 0,
        }
    }

    pub fn next(&mut self) -> &'a Token<'a> {
        match self.state {
            TokenStreamState::NotExpired(index) => {
                let token = &self.tokens[index];
                self.line = token.line;
                self.set_state(index + 1);
                token
            }
            TokenStreamState::Expired(token) => token,
        }
    }

    pub fn peek(&self) -> &'a Token<'a> {
        match self.state {
            TokenStreamState::NotExpired(index) => &self.tokens[index],
            TokenStreamState::Expired(token) => token,
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    fn set_state(&mut self, index: usize) {
        if index < self.tokens.len() {
            self.state = TokenStreamState::NotExpired(index);
        } else {
            self.state = TokenStreamState::Expired(&self.tokens[self.tokens.len() - 1]);
        }
    }
}
