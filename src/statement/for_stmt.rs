use std::io::Write;

use crate::{expr::ExprAst, mac::tt};

use super::{Runtime, RuntimeError, StmtAst, StmtParseError, StmtParser};

#[derive(Clone, Debug, PartialEq)]
pub struct For {
    initializer: Option<Box<StmtAst>>,
    condition: Option<ExprAst>,
    increment: Option<ExprAst>,
    body: Box<StmtAst>,
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

impl<W: Write> Runtime<W> {
    pub(super) fn run_for(&self, for_stmt: For) -> Result<(), RuntimeError> {
        let For {
            initializer,
            condition,
            increment,
            body,
        } = for_stmt;

        if let Some(init) = initializer {
            self.run(*init)?;
        }

        loop {
            if let Some(condition) = condition.as_ref() {
                let value = self.evaluate(condition)?;
                if !value.is_literal_and(|l| l.is_truthy()) {
                    break;
                }
            }

            self.run(*body.clone())?;

            if let Some(increment) = increment.as_ref() {
                self.evaluate(increment)?;
            }
        }

        Ok(())
    }
}
