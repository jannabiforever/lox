use crate::{
    parse::{Assign, ExprAst},
    run::{error::StmtParseError, stmt_ast::VarDecl},
};

use super::StmtParser;

impl StmtParser<'_, '_> {
    pub fn parse_var_decl(&mut self) -> Result<VarDecl, StmtParseError> {
        self.token_stream.next(); // consume the 'var' token.
        let following = self.parse_following_expression()?;

        match following {
            // e.g. var x;
            ExprAst::Variable(_) => Ok(VarDecl {
                var: following,
                value: ExprAst::default(),
            }),
            // e.g. var x = 1;
            ExprAst::Assign(Assign { assignee, value }) => Ok(VarDecl {
                var: *assignee.clone(),
                value: *value.clone(),
            }),
            _ => Err(StmtParseError::InvalidVarDecl(following.to_string())),
        }
    }
}
