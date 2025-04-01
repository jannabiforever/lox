mod print;

use super::{error::StmtParseError, stmt_ast::StmtAst};
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

impl<'a> StmtParser<'a, '_> {
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
            _ => todo!("Parse other statements"),
        }
    }

    fn parse_following_expression(&mut self) -> Result<ExprAst, StmtParseError> {
        ExprParser::new(&mut self.token_stream)
            .parse()
            .map_err(Into::into)
    }
}
