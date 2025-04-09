use std::{cell::RefCell, io::Write, rc::Rc};

use crate::{env::Runnable, expr::ExprAst, mac::tt, Env, Evaluatable};

use super::{RuntimeError, StmtAst, StmtParseError, StmtParser};

#[derive(Clone, Debug, PartialEq)]
pub struct For {
    initializer: Option<Box<StmtAst>>,
    condition: Option<ExprAst>,
    increment: Option<ExprAst>,
    body: Box<StmtAst>,
}

impl Runnable for For {
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<(), RuntimeError> {
        let For {
            initializer,
            condition,
            increment,
            body,
        } = self;

        if let Some(init) = initializer {
            init.run(env.clone())?
        }

        loop {
            if let Some(condition) = condition.as_ref() {
                let value = condition.eval(env.clone())?;
                if !value.is_literal_and(|l| l.is_truthy()) {
                    break;
                }
            }

            body.run(env.clone())?;

            if let Some(increment) = increment.as_ref() {
                increment.eval(env.clone())?;
            }
        }

        Ok(())
    }
}

impl StmtParser<'_, '_> {
    pub(super) fn parse_for(&mut self) -> Result<For, StmtParseError> {
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
            | StmtAst::Block(_)) => Box::new(allowed),
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
