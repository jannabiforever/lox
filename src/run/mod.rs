mod block;
mod error;
mod expression;
mod for_stmt;
mod if_stmt;
mod print;
mod var_decl;
mod while_stmt;

use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

pub(crate) use self::block::Block;
pub(crate) use self::error::{RuntimeError, StmtParseError};
pub(crate) use self::expression::Expression;
pub(crate) use self::for_stmt::For;
pub(crate) use self::if_stmt::If;
pub(crate) use self::print::Print;
pub(crate) use self::var_decl::VarDecl;
pub(crate) use self::while_stmt::While;

use crate::env::Environment;
use crate::error::{IntoLoxError, LoxError};
use crate::evaluate::Evaluator;
use crate::literal::Literal;
use crate::mac::{impl_from, rc_rc};
use crate::{
    parse::{ExprAst, ExprParser},
    tokenize::tt,
    TokenStream,
};

/// Statement AST.
#[derive(Debug, Clone)]
pub(crate) enum StmtAst {
    Expression(Expression),
    Print(Print),
    VarDecl(VarDecl),
    Block(Block),
    If(If),
    While(While),
    For(For),
}

impl_from!(StmtAst: Expression, Print, VarDecl, Block, If, While, For);

/// Parser for statement AST.
/// Generic 'a is for the source's lifetime.
/// Generic 'b is for the lifetime of mutable reference of token stream.
pub(crate) struct StmtParser<'a, 'b> {
    pub(crate) token_stream: &'b mut TokenStream<'a>,
}

impl<'a, 'b> StmtParser<'a, 'b> {
    pub fn new(token_stream: &'b mut TokenStream<'a>) -> Self {
        StmtParser { token_stream }
    }
}

impl StmtParser<'_, '_> {
    /// Parses whole source code into vector of AST.
    pub(crate) fn parse_all(mut self) -> Result<Vec<StmtAst>, LoxError<StmtParseError>> {
        let mut statements = Vec::new();
        while !self.token_stream.expired() {
            let stmt = self
                .parse()
                .map_err(|err| err.error(self.token_stream.line()))?;
            statements.push(stmt);
        }
        Ok(statements)
    }

    /// Parses the following AST.
    pub(crate) fn parse(&mut self) -> Result<StmtAst, StmtParseError> {
        match self.token_stream.peek().token_type {
            tt!("print") => self.parse_print().map(Into::into),
            tt!("var") => self.parse_var_decl().map(Into::into),
            tt!("{") => self.parse_block().map(Into::into),
            tt!("if") => self.parse_if().map(Into::into),
            tt!("while") => self.parse_while().map(Into::into),
            tt!("for") => self.parse_for().map(Into::into),
            _ => self.parse_expression_stmt().map(Into::into),
        }
    }

    /// Lent its own token stream's mutable reference to expression parser,
    /// and parse following expression.
    fn parse_following_expression(&mut self) -> Result<ExprAst, StmtParseError> {
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

/// Lox Runtime.
///
/// Represents the runtime environment for the interpreter.
///
/// The `Runtime` struct holds a reference to the current scope's environment,
/// which is shared across the execution of the program. Environment
/// contains variables and their associated values, and it is managed using
/// reference counting and interior mutability to allow for safe and flexible
/// updates during runtime.
pub struct Runtime<W: Write> {
    stdout: Rc<RefCell<W>>,
    env: Rc<RefCell<Environment>>,
}

impl<W: Write> Runtime<W> {
    pub fn new(stdout: Rc<RefCell<W>>) -> Self {
        Self {
            stdout,
            env: rc_rc!(Environment::new()),
        }
    }

    pub fn from_env(stdout: Rc<RefCell<W>>, env: Rc<RefCell<Environment>>) -> Self {
        Self { stdout, env }
    }

    fn child_runtime(&self) -> Self {
        let child_env = Environment::from_parent(&self.env);
        Self::from_env(self.stdout.clone(), rc_rc!(child_env))
    }

    pub fn run(&self, stmt: StmtAst) -> Result<(), RuntimeError> {
        match stmt {
            StmtAst::Print(print) => self.run_print(print)?,
            StmtAst::Expression(expr) => self.run_expression(expr)?,
            StmtAst::VarDecl(var_decl) => self.run_var_decl(var_decl)?,
            StmtAst::Block(block) => self.run_block(block)?,
            StmtAst::If(if_stmt) => self.run_if(if_stmt)?,
            StmtAst::While(while_stmt) => self.run_while(while_stmt)?,
            StmtAst::For(for_stmt) => self.run_for(for_stmt)?,
        }

        Ok(())
    }

    fn evaluate(&self, expr: &ExprAst) -> Result<Literal, RuntimeError> {
        self.evaluator().eval(expr).map_err(Into::into)
    }

    fn evaluator(&self) -> Evaluator {
        Evaluator::with_env(self.env.clone())
    }

    fn assignable_key(&self, expr: &ExprAst) -> Result<String, RuntimeError> {
        match expr {
            ExprAst::Variable(variable) => Ok(variable.clone()),
            rest => Err(RuntimeError::InvalidAssignmentTarget(rest.to_string())),
        }
    }
}
