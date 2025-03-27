use crate::{
    error::{ErrorReporter, ResultWithLine, WithLine},
    literal::{Literal, Number},
    tokenize::{tt, Token},
};

use super::{expr_ast::ExprAst, ParseError};

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

    pub(crate) fn parse(&mut self) -> ResultWithLine<ExprAst, ParseError> {
        let peeked = self.peek();
        let src = peeked.src;
        match peeked.token_type {
            tt!("nil") => {
                self.next();
                self.wrap(Ok(ExprAst::Literal(Literal::Nil)))
            }
            tt!("true") => {
                self.next();
                self.wrap(Ok(ExprAst::Literal(Literal::Boolean(true))))
            }
            tt!("false") => {
                self.next();
                self.wrap(Ok(ExprAst::Literal(Literal::Boolean(false))))
            }
            tt!("number") => {
                self.next();
                let num = src.parse::<Number>().unwrap();
                self.wrap(Ok(ExprAst::Literal(Literal::Number(num))))
            }
            _ => todo!(),
        }
    }

    /// Get the next token.
    /// Note: No need to return line number, because it is hanlded by [`ErrorReporter`] trait.
    fn next(&mut self) -> &Token<'a> {
        let token = self.tokens.get(self.current).unwrap().as_ref().into_inner();
        self.current += 1;
        token
    }

    fn peek(&self) -> &Token<'a> {
        self.tokens.get(self.current).unwrap().as_ref().into_inner()
    }
}

impl ErrorReporter<ParseError> for ExprParser<'_, '_> {
    fn line(&self) -> usize {
        // current token's line number
        self.tokens[self.current].line
    }
}
