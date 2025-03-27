use crate::{
    error::{ErrorReporter, LoxError, ResultWithLine, WithLine},
    literal::{Literal, Number},
    tokenize::{tt, Token, TokenType},
};

use super::{
    expr_ast::{ExprAst, Grouping},
    ParseError,
};

pub(crate) struct ExprParser<'a, 'b> {
    /// Note: For reporting errors, we need to know the line number.
    tokens: &'b [WithLine<Token<'a>>],
    /// Token index.
    current: usize,
}

impl<'a, 'b> ExprParser<'a, 'b> {
    pub(crate) fn new(tokens: &'b [WithLine<Token<'a>>]) -> Self {
        Self { tokens, current: 0 }
    }

    pub(crate) fn parse_with_line(&mut self) -> ResultWithLine<ExprAst, LoxError> {
        let expr = self.parse().map_err(|e| e.into());
        self.wrap(expr)
    }

    pub(crate) fn parse(&mut self) -> Result<ExprAst, ParseError> {
        let peeked = self.peek();
        let src = peeked.src;
        match peeked.token_type {
            tt!("nil") => {
                self.next();
                Ok(ExprAst::Literal(Literal::Nil))
            }
            tt!("true") => {
                self.next();
                Ok(ExprAst::Literal(Literal::Boolean(true)))
            }
            tt!("false") => {
                self.next();
                Ok(ExprAst::Literal(Literal::Boolean(false)))
            }
            tt!("number") => {
                self.next();
                let num = src.parse::<Number>().unwrap();
                Ok(ExprAst::Literal(Literal::Number(num)))
            }
            tt!("string") => {
                self.next();
                let src = src.trim_matches('"');
                Ok(ExprAst::Literal(Literal::String(src.to_string())))
            }
            tt!("(") => {
                self.next();
                let grouping = Grouping {
                    inner: Box::new(self.parse()?),
                };
                self.expect(tt!(")"))?;
                Ok(grouping.into())
            }
            _ => todo!(),
        }
    }

    /// Get the next token. Panics if the end of the tokens is reached.
    /// Note: No need to return line number, because it is hanlded by [`ErrorReporter`] trait.
    fn next(&mut self) -> &Token<'a> {
        let token = self.tokens.get(self.current).unwrap().inner_ref();
        self.current += 1;
        token
    }

    /// Expect the next token to be of a certain type.
    fn expect(&mut self, token_type: TokenType) -> Result<&Token<'a>, ParseError> {
        let token = self.next();
        if token.token_type == token_type {
            Ok(token)
        } else {
            todo!("")
        }
    }

    fn peek(&self) -> &Token<'a> {
        self.tokens.get(self.current).unwrap().inner_ref()
    }
}

impl ErrorReporter<ParseError> for ExprParser<'_, '_> {
    fn line(&self) -> usize {
        // current token's line number
        self.tokens[self.current].line
    }
}
