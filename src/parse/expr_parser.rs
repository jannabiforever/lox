use crate::{
    error::{ErrorReporter, ResultWithLine, WithLine},
    literal::Literal,
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
        match self.peek().as_ref().into_inner().token_type {
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
            _ => todo!(),
        }
    }

    fn next(&mut self) -> &WithLine<Token<'a>> {
        let token = self.tokens.get(self.current).unwrap();
        self.current += 1;
        token
    }

    fn peek(&self) -> &WithLine<Token<'a>> {
        self.tokens.get(self.current).unwrap()
    }
}

impl ErrorReporter<ParseError> for ExprParser<'_, '_> {
    fn line(&self) -> usize {
        // current token's line number
        self.tokens[self.current].line
    }
}
