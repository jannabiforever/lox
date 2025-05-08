use std::{cell::RefCell, io::Write, rc::Rc};

use super::{RuntimeError, StmtAst, StmtParseError, StmtParser};
use crate::{
    env::Runnable, error::LoxError, expr::ExprAst, literal::LoxValue, mac::tt, Env, Evaluatable,
};

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Clone, Debug, PartialEq)]
pub struct For<'a> {
    initializer: Option<Box<StmtAst<'a>>>,
    condition: Option<ExprAst<'a>>,
    increment: Option<ExprAst<'a>>,
    body: Box<StmtAst<'a>>,
}

impl<'a> Runnable<'a> for For<'a> {
    fn run<W: Write>(
        &self,
        env: Rc<RefCell<Env<'a>>>,
        stdout: &mut W,
    ) -> Result<Option<LoxValue<'a>>, LoxError<RuntimeError>> {
        let For {
            initializer,
            condition,
            increment,
            body,
        } = self;

        if let Some(init) = initializer {
            init.run(env.clone(), stdout)?;
        }

        loop {
            if let Some(condition) = condition.as_ref() {
                let value = condition.eval(env.clone(), stdout)?;
                if !value.is_literal_and(|l| l.is_truthy()) {
                    break;
                }
            }

            if let Some(value) = body.run(env.clone(), stdout)? {
                return Ok(Some(value));
            }

            if let Some(increment) = increment.as_ref() {
                increment.eval(env.clone(), stdout)?;
            }
        }

        Ok(None)
    }

    fn line(&self) -> usize {
        self.body.line()
    }
}

impl<'a> StmtParser<'a, '_> {
    pub(super) fn parse_for(&mut self) -> Result<For<'a>, StmtParseError> {
        self.token_stream.next(); // Consume 'for'.
        self.expect_opening_paren()?;

        let initializer = if self.token_stream.peek().token_type == tt!(";") {
            self.token_stream.next();
            None
        } else {
            match self.parse()? {
                var_decl @ StmtAst::VarDecl(_) => Some(Box::new(var_decl)),
                expression @ StmtAst::Expression(_) => Some(Box::new(expression)),
                rest => {
                    return Err(StmtParseError::InvalidForStmtInitializer(format!(
                        "{rest:?}"
                    )))
                }
            }
        };

        let condition = if self.token_stream.peek().token_type == tt!(";") {
            None
        } else {
            let condition = self.parse_following_expression()?;
            Some(condition)
        };
        self.expect_semicolon()?;

        let increment = if self.token_stream.peek().token_type == tt!(")") {
            None
        } else {
            let increment = self.parse_following_expression()?;
            Some(increment)
        };
        self.expect_closing_paren()?;

        let body = match self.parse()? {
            allowed @ (StmtAst::Expression(_)
            | StmtAst::For(_)
            | StmtAst::If(_)
            | StmtAst::Print(_)
            | StmtAst::While(_)
            | StmtAst::Block(_)
            | StmtAst::Return(_)) => Box::new(allowed),
            rest => return Err(StmtParseError::InvalidForStmtBody(format!("{rest:?}"))),
        };

        Ok(For {
            initializer,
            condition,
            increment,
            body,
        })
    }
}
