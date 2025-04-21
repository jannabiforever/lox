mod block;
mod error;
mod expression;
mod for_stmt;
mod function_def;
mod if_stmt;
mod print;
mod return_stmt;
mod var_decl;
mod while_stmt;

use std::{cell::RefCell, io::Write, rc::Rc};

pub(crate) use self::{
    block::Block, error::StmtParseError, expression::Expression, for_stmt::For,
    function_def::FunctionDef, if_stmt::If, print::Print, return_stmt::Return, var_decl::VarDecl,
    while_stmt::While,
};
use crate::{
    env::{Env, Runnable, RuntimeError},
    error::{IntoLoxError, LoxError},
    expr::{ExprAst, ExprParser},
    literal::LoxValue,
    mac::{impl_from, tt},
    TokenStream,
};

/// Statement AST.
/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StmtAst<'a> {
    Expression(Expression<'a>),
    Print(Print<'a>),
    VarDecl(VarDecl<'a>),
    Block(Block<'a>),
    If(If<'a>),
    While(While<'a>),
    For(For<'a>),
    FunctionDef(FunctionDef<'a>),
    Return(Return<'a>),
}

impl<'a> Runnable<'a> for StmtAst<'a> {
    fn run<W: Write>(
        &self,
        env: Rc<RefCell<Env<'a, W>>>,
    ) -> Result<Option<LoxValue<'a>>, LoxError<RuntimeError>> {
        match self {
            Self::Print(print) => print.run(env),
            Self::Expression(expression) => expression.run(env),
            Self::VarDecl(var_decl) => var_decl.run(env),
            Self::Block(block) => block.run(env),
            Self::If(if_stmt) => if_stmt.run(env),
            Self::While(while_stmt) => while_stmt.run(env),
            Self::For(for_stmt) => for_stmt.run(env),
            Self::FunctionDef(function_def) => function_def.run(env),
            Self::Return(return_stmt) => return_stmt.run(env),
        }
    }

    fn line(&self) -> usize {
        match self {
            Self::Print(v) => v.line(),
            Self::Expression(v) => v.line(),
            Self::VarDecl(v) => v.line(),
            Self::Block(v) => v.line(),
            Self::If(v) => v.line(),
            Self::While(v) => v.line(),
            Self::For(v) => v.line(),
            Self::FunctionDef(v) => v.line(),
            Self::Return(v) => v.line(),
        }
    }
}

impl_from!('a StmtAst: Expression, Print, VarDecl, Block, If, While, For, FunctionDef, Return);

/// Parser for statement AST.
/// Generic 'a is for the source's lifetime.
/// Generic 'b is for the lifetime of mutable reference of token stream.
pub(crate) struct StmtParser<'a, 'mr> {
    pub(crate) token_stream: &'mr mut TokenStream<'a>,
}

impl<'a, 'mr> StmtParser<'a, 'mr> {
    pub fn new(token_stream: &'mr mut TokenStream<'a>) -> Self {
        StmtParser { token_stream }
    }
}

impl<'a> StmtParser<'a, '_> {
    /// Parses whole source code into vector of AST.
    pub(crate) fn parse_all(mut self) -> Result<Vec<StmtAst<'a>>, LoxError<StmtParseError>> {
        let mut statements = Vec::new();
        while !self.token_stream.expired() {
            let stmt = self
                .parse()
                .map_err(|err| err.at(self.token_stream.line()))?;
            statements.push(stmt);
        }
        Ok(statements)
    }

    /// Parses the following AST.
    pub(crate) fn parse(&mut self) -> Result<StmtAst<'a>, StmtParseError> {
        match self.token_stream.peek().token_type {
            tt!("print") => self.parse_print().map(Into::into),
            tt!("var") => self.parse_var_decl().map(Into::into),
            tt!("{") => self.parse_block().map(Into::into),
            tt!("if") => self.parse_if().map(Into::into),
            tt!("while") => self.parse_while().map(Into::into),
            tt!("for") => self.parse_for().map(Into::into),
            tt!("fun") => self.parse_function_def().map(Into::into),
            tt!("return") => self.parse_return().map(Into::into),
            _ => self.parse_expression_stmt().map(Into::into),
        }
    }

    /// Lent its own token stream's mutable reference to expression parser,
    /// and parse following expression.
    fn parse_following_expression(&mut self) -> Result<ExprAst<'a>, StmtParseError> {
        ExprParser::new(self.token_stream)
            .parse()
            .map_err(Into::into)
    }

    /// Consumes a token, and expect it to be semicolon or return err.
    fn expect_semicolon(&mut self) -> Result<(), StmtParseError> {
        self.token_stream
            .expect(tt!(";"))
            .map_err(|unexpected_token| {
                StmtParseError::ExpectedSemicolon(unexpected_token.src.to_string())
            })?;

        Ok(())
    }

    fn expect_opening_paren(&mut self) -> Result<(), StmtParseError> {
        self.token_stream
            .expect(tt!("("))
            .map_err(|unexpected_token| {
                StmtParseError::ExpectedOpeningParentheses(unexpected_token.src.to_string())
            })?;

        Ok(())
    }

    fn expect_closing_paren(&mut self) -> Result<(), StmtParseError> {
        self.token_stream
            .expect(tt!(")"))
            .map_err(|unexpected_token| {
                StmtParseError::ExpectedClosingParentheses(unexpected_token.src.to_string())
            })?;

        Ok(())
    }
}
