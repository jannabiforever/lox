mod print;
mod var_decl;

use super::{
    error::StmtParseError,
    stmt_ast::{Expression, StmtAst},
};
use crate::{
    parse::{ExprAst, ExprParser},
    tokenize::tt,
    TokenStream,
};

pub(crate) struct StmtParser<'a, 'b> {
    pub(crate) token_stream: &'b mut TokenStream<'a>,
}

impl<'a, 'b> StmtParser<'a, 'b> {
    pub fn new(token_stream: &'b mut TokenStream<'a>) -> Self {
        StmtParser { token_stream }
    }
}

impl StmtParser<'_, '_> {
    pub fn parse_all(mut self) -> Result<Vec<StmtAst>, StmtParseError> {
        let mut statements = Vec::new();
        while !self.token_stream.expired() {
            let stmt = self.parse()?;
            statements.push(stmt);
        }
        Ok(statements)
    }

    pub fn parse(&mut self) -> Result<StmtAst, StmtParseError> {
        match self.token_stream.peek().token_type {
            tt!("print") => self.parse_print().map(Into::into),
            tt!("var") => self.parse_var_decl().map(Into::into),
            _ => {
                // Expression statement.
                let expr = self.parse_following_expression()?;
                self.token_stream
                    .expect(tt!(";"))
                    .map_err(|unexpected_token| {
                        StmtParseError::ExpectedSemicolon(unexpected_token.to_string())
                    })?;
                Ok(Expression { expr }.into())
            }
        }
    }

    fn parse_following_expression(&mut self) -> Result<ExprAst, StmtParseError> {
        ExprParser::new(self.token_stream)
            .parse()
            .map_err(Into::into)
    }
}
