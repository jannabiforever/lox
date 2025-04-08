use std::io::Write;

use crate::{
    expr::{Assign, ExprAst},
    literal::Literal,
    statement::error::StmtParseError,
};

use super::{Runtime, RuntimeError, StmtParser};

#[derive(Debug, Clone)]
pub(crate) struct VarDecl {
    pub(crate) var: ExprAst,
    pub(crate) value: Option<ExprAst>,
}

impl StmtParser<'_, '_> {
    pub fn parse_var_decl(&mut self) -> Result<VarDecl, StmtParseError> {
        self.token_stream.next(); // consume the 'var' token.
        let following = self.parse_following_expression()?;

        let result = match following {
            // e.g. var x;
            ExprAst::Variable(_) => Ok(VarDecl {
                var: following,
                value: None,
            }),
            // e.g. var x = 1;
            ExprAst::Assign(Assign { assignee, value }) => Ok(VarDecl {
                var: *assignee.clone(),
                value: Some(*value.clone()),
            }),
            _ => Err(StmtParseError::InvalidVarDecl(following.to_string())),
        };

        self.expect_semicolon()?;

        result
    }
}

impl<W: Write> Runtime<W> {
    pub(super) fn run_var_decl(&self, var_decl: VarDecl) -> Result<(), RuntimeError> {
        let VarDecl { var, value } = var_decl;
        let var = self.assignable_key(&var)?;
        let value = match value {
            Some(value) => self.evaluate(&value)?,
            None => Literal::Nil,
        };
        self.env.borrow_mut().set(&var, value);
        Ok(())
    }
}
