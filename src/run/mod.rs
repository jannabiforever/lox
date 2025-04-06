mod error;
mod expression;
mod print;
mod var_decl;

use std::cell::RefCell;
use std::rc::Rc;

pub(crate) use self::error::{RuntimeError, StmtParseError};
pub(crate) use self::expression::Expression;
pub(crate) use self::print::Print;
pub(crate) use self::var_decl::VarDecl;

use crate::env::Environment;
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
}

impl_from!(StmtAst: Expression, Print, VarDecl);

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
    pub fn parse_all(mut self) -> Result<Vec<StmtAst>, StmtParseError> {
        let mut statements = Vec::new();
        while !self.token_stream.expired() {
            let stmt = self.parse()?;
            statements.push(stmt);
        }
        Ok(statements)
    }

    /// Parses the following AST.
    pub fn parse(&mut self) -> Result<StmtAst, StmtParseError> {
        match self.token_stream.peek().token_type {
            tt!("print") => self.parse_print().map(Into::into),
            tt!("var") => self.parse_var_decl().map(Into::into),
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
}

/// Lox Runtime.
///
/// Represents the runtime environment for the interpreter.
///
/// The `Runtime` struct holds a reference to the global environment, which
/// is shared across the execution of the program. The global environment
/// contains variables and their associated values, and it is managed using
/// reference counting and interior mutability to allow for safe and flexible
/// updates during runtime.
pub struct Runtime {
    global_env: Rc<RefCell<Environment>>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            global_env: rc_rc!(Environment::new()),
        }
    }

    pub fn run(&self, stmt: StmtAst) -> Result<(), RuntimeError> {
        match stmt {
            StmtAst::Print(print) => self.run_print(print)?,
            StmtAst::Expression(expr) => self.run_expression(expr)?,
            StmtAst::VarDecl(var_decl) => self.run_var_decl(var_decl)?,
        }

        Ok(())
    }

    fn evaluate(&self, expr: &ExprAst) -> Result<Literal, RuntimeError> {
        self.evaluator().eval(expr).map_err(Into::into)
    }

    fn evaluator(&self) -> Evaluator {
        Evaluator::with_env(self.global_env.clone())
    }

    fn assignable_key(&self, expr: &ExprAst) -> Result<String, RuntimeError> {
        match expr {
            ExprAst::Variable(variable) => Ok(variable.clone()),
            rest => Err(RuntimeError::InvalidAssignmentTarget(rest.to_string())),
        }
    }
}
