mod unit;

use unit::ForStmt;
pub(crate) use unit::{
    BlockStmt, ExprStmt, FuncDeclStmt, IfStmt, PrintStmt, ReturnStmt, Stmt, VarDeclStmt, WhileStmt,
};

use crate::{
    error::{ASTError::*, ErrorReporter, LoxError, WithLine},
    expr_ast::{Expr, ExprASTParser},
    lex::{Token, TokenType, lexer::Lexer, tt},
};

pub(crate) fn parse_stmt_ast(source: &str) -> Vec<WithLine<Result<Stmt, LoxError>>> {
    let mut parser = StmtASTParser::new(Lexer::new(source));
    parser.parse_stmts()
}

struct StmtASTParser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> StmtASTParser<'a> {
    fn next(&mut self) -> Result<Token<'a>, LoxError> {
        let token = self.lexer.scan_token()?;
        Ok(token)
    }

    fn peek(&self) -> Result<Token<'a>, LoxError> {
        let token = self.lexer.clone().scan_token()?;
        Ok(token)
    }

    /// Consumes the next token and throw error if it is not the expected.
    fn expect(&mut self, expected: TokenType) -> Result<Token<'a>, LoxError> {
        let next_token = self.next()?;
        if next_token.token_type == expected {
            Ok(next_token)
        } else {
            Err(match &expected {
                tt!(")") => ExpectedClosingDelimiter(')'),
                tt!("}") => ExpectedClosingDelimiter('}'),
                _ => ExpectedToken(format!("{:?}", expected)),
            }
            .into())
        }
    }

    /// Consumes the next token if it is the expected token.
    /// Returns true if the token was consumed.
    fn eat(&mut self, expected: TokenType) -> bool {
        if self
            .peek()
            .map(|t| t.token_type == expected)
            .unwrap_or(false)
        {
            self.next().unwrap();
            true
        } else {
            false
        }
    }
}

impl<'a> StmtASTParser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    fn parse_stmts(&mut self) -> Vec<WithLine<Result<Stmt, LoxError>>> {
        let mut stmts = Vec::new();
        while !self.peek().is_ok_and(|t| t.token_type == tt!("")) {
            let stmt = match self.parse_stmt() {
                Ok(stmt) => self.get_lox_ok(stmt),
                Err(err) => self.get_lox_err(err),
            };
            stmts.push(stmt);
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Result<Stmt, LoxError> {
        let peeked_token = self.peek()?;
        let stmt = match &peeked_token.token_type {
            tt!("print") => self.parse_print_stmt()?.into(),
            tt!("var") => self.parse_var_decl_stmt()?.into(),
            tt!("{") => self.parse_block_stmt()?.into(),
            tt!("if") => self.parse_if_stmt()?.into(),
            tt!("while") => self.parse_while_stmt()?.into(),
            tt!("fun") => self.parse_func_decl_stmt()?.into(),
            tt!("return") => self.parse_return_stmt()?.into(),
            tt!("for") => self.parse_for_stmt()?.into(),
            _ => self.parse_expr_stmt()?.into(),
        };

        Ok(stmt)
    }

    fn parse_expr(&mut self) -> Result<Expr, LoxError> {
        let mut expr_parser = ExprASTParser::new(&mut self.lexer);
        expr_parser.parse_expr()
    }

    fn parse_print_stmt(&mut self) -> Result<PrintStmt, LoxError> {
        self.expect(tt!("print"))?;
        let expr = self.parse_expr()?;
        self.expect(tt!(";"))?;

        Ok(expr.into())
    }

    fn parse_var_decl_stmt(&mut self) -> Result<VarDeclStmt, LoxError> {
        self.expect(tt!("var"))?;
        let name = self.expect(tt!("identifier"))?;

        let initializer = if self.eat(tt!("=")) {
            Some(self.parse_expr()?)
        } else {
            None
        };

        self.expect(tt!(";"))?;

        Ok((name.source.to_string(), initializer).into())
    }

    fn parse_block_stmt(&mut self) -> Result<BlockStmt, LoxError> {
        self.expect(tt!("{"))?;
        let mut stmts = Vec::new();
        while !self.eat(tt!("}")) {
            if let Ok(tt!("")) = self.peek().map(|t| t.token_type) {
                return Err(ExpectedClosingDelimiter('}').into());
            }
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts.into())
    }

    fn parse_if_stmt(&mut self) -> Result<IfStmt, LoxError> {
        self.expect(tt!("if"))?;
        self.expect(tt!("("))?;
        let condition = self.parse_expr()?;
        self.expect(tt!(")"))?;

        let then_branch = Box::new(self.parse_stmt()?);
        let else_branch = if self.eat(tt!("else")) {
            Some(self.parse_stmt()?)
        } else {
            None
        };

        Ok((condition, *then_branch, else_branch).into())
    }

    fn parse_while_stmt(&mut self) -> Result<WhileStmt, LoxError> {
        self.expect(tt!("while"))?;
        self.expect(tt!("("))?;
        let condition = self.parse_expr()?;
        self.expect(tt!(")"))?;

        let body = self.parse_stmt()?;

        Ok((condition, body).into())
    }

    fn parse_func_decl_stmt(&mut self) -> Result<FuncDeclStmt, LoxError> {
        self.expect(tt!("fun"))?;
        let name = self.expect(tt!("identifier"))?;
        self.expect(tt!("("))?;

        let mut params = Vec::new();
        if !self.eat(tt!(")")) {
            loop {
                let param = self.expect(tt!("identifier"))?;
                params.push(param.source.to_string());
                if !self.eat(tt!(",")) {
                    break;
                }
            }
            self.expect(tt!(")"))?;
        }

        let body = self.parse_block_stmt()?.stmts;

        Ok((name.source.to_string(), params, body).into())
    }

    fn parse_return_stmt(&mut self) -> Result<ReturnStmt, LoxError> {
        self.expect(tt!("return"))?;
        let value = if self.eat(tt!(";")) {
            None
        } else {
            Some(self.parse_expr()?)
        };

        Ok(value.into())
    }

    fn parse_expr_stmt(&mut self) -> Result<ExprStmt, LoxError> {
        let expr = self.parse_expr()?;
        self.expect(tt!(";"))?;

        Ok(expr.into())
    }

    fn parse_for_stmt(&mut self) -> Result<ForStmt, LoxError> {
        self.expect(tt!("for"))?;
        self.expect(tt!("("))?;

        let initializer: Option<Stmt> = match self.peek()?.token_type {
            tt!("var") => Some(self.parse_var_decl_stmt()?.into()),
            tt!(";") => {
                self.expect(tt!(";"))?;
                None
            }
            _ => Some(self.parse_expr_stmt()?.into()),
        };

        let condition = if !self.eat(tt!(";")) {
            let condition = Some(self.parse_expr()?);
            self.expect(tt!(";"))?;
            condition
        } else {
            None
        };

        let increment = if !self.eat(tt!(")")) {
            let increment = Some(self.parse_expr()?);
            self.expect(tt!(")"))?;
            increment
        } else {
            None
        };

        let body = self.parse_stmt()?;

        Ok((initializer, condition, increment, body).into())
    }
}

impl ErrorReporter<LoxError> for StmtASTParser<'_> {
    fn line(&self) -> usize {
        self.lexer.line()
    }
}
