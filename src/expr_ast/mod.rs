//! Parses the AST from tokens.
pub(crate) mod owned_token;
mod unit;

use unit::Variable;

pub use self::unit::{Assign, Binary, BinaryOp, Expr, Grouping, Unary, UnaryOp};
use crate::{
    error::{ASTError, ErrorReporter, LoxError, WithLine},
    lex::{Token, TokenType, lexer::Lexer, tt},
    literal::Literal,
};

pub fn parse_expr_ast(source: &str) -> WithLine<Result<Expr, LoxError>> {
    let mut lexer = Lexer::new(source);
    let mut parser = ExprASTParser::new(&mut lexer);
    parser.parse_expr_with_line()
}

pub(crate) struct ExprASTParser<'a, 'b> {
    lexer: &'b mut Lexer<'a>,
}

impl<'a, 'b> ExprASTParser<'a, 'b> {
    pub(crate) fn new(lexer: &'b mut Lexer<'a>) -> Self {
        Self { lexer }
    }
}

impl<'a> ExprASTParser<'a, '_> {
    fn next(&mut self) -> Result<Token<'a>, LoxError> {
        let token = self.lexer.scan_token()?;
        Ok(token)
    }

    fn peek(&self) -> Result<Token<'a>, LoxError> {
        let token = self.lexer.clone().scan_token()?;
        Ok(token)
    }

    /// Consumes the next token and checks if it is the expected token.
    fn expect(&mut self, expected: TokenType) -> Result<Token, LoxError> {
        let next_token = self.lexer.scan_token()?;
        if next_token.token_type == expected {
            Ok(next_token)
        } else {
            todo!("Error handling.")
        }
    }
}

impl ExprASTParser<'_, '_> {
    pub(crate) fn parse_expr_with_line(&mut self) -> WithLine<Result<Expr, LoxError>> {
        let result = self.parse_expr();
        self.wrap(result)
    }

    pub(crate) fn parse_expr(&mut self) -> Result<Expr, LoxError> {
        let mut lhs = self.parse_expr_unit()?;

        loop {
            let peeked_token = self.peek()?;
            match &peeked_token.token_type {
                // End of expression, so don't consume the token.
                tt!("") | tt!(";") | tt!(")") | tt!("}") | tt!(",") => {
                    break;
                }
                // Assignment
                tt!("=") => {
                    self.parse_assignment(&mut lhs)?;
                    break;
                }
                // Binary operator.
                tt!("+")
                | tt!("-")
                | tt!("*")
                | tt!("/")
                | tt!("==")
                | tt!("!=")
                | tt!(">")
                | tt!(">=")
                | tt!("<")
                | tt!("<=")
                | tt!("and")
                | tt!("or") => {
                    self.parse_binary(&mut lhs)?;
                }
                _ => {
                    todo!("Error handling.");
                }
            }
        }

        Ok(lhs)
    }

    /// Note: next token must be a binary operator.
    /// This function consumes the next token and binds it to the left-hand side expression.
    fn parse_binary(&mut self, lhs: &mut Expr) -> Result<(), LoxError> {
        let new_op = BinaryOp::from_token(&self.next()?).unwrap();
        let rhs = self.parse_expr_unit()?;

        if let Expr::Binary(binary) = lhs {
            // Compare the precedence of the current operator with the operator in the binary expression.
            binary.bind(new_op, rhs);
        } else {
            *lhs = Binary {
                left: Box::new(lhs.clone()),
                op: new_op,
                right: Box::new(rhs),
            }
            .into();
        }

        Ok(())
    }

    /// Consumes the first ahead expression unit.
    fn parse_expr_unit(&mut self) -> Result<Expr, LoxError> {
        let cur_token = self.next()?;

        let source = cur_token.source;
        match &cur_token.token_type {
            // Literal expression.
            tt!("number") => Ok(Literal::number_from_source(source.to_string()).into()),
            tt!("string") => Ok(Literal::string_from_source(source.to_string()).into()),
            tt!("true") => Ok(Literal::Boolean(true).into()),
            tt!("false") => Ok(Literal::Boolean(false).into()),
            tt!("nil") => Ok(Literal::Nil.into()),
            // Grouping expression.
            tt!("(") => {
                let inner_expr = self.parse_expr()?;
                self.expect(TokenType::RightParen)?;

                let grouping_expr = Grouping {
                    inner: Box::new(inner_expr),
                };

                Ok(grouping_expr.into())
            }
            tt!("identifier") => Ok(Expr::Variable(Variable {
                name: cur_token.to_owned(),
            })),
            // Unary expression.
            tt!("-") | tt!("!") => {
                let op = UnaryOp::from_token(&cur_token).unwrap();
                let right_expr = self.parse_expr_unit()?;

                let unary_expr = Unary {
                    op,
                    right: Box::new(right_expr),
                };

                Ok(unary_expr.into())
            }
            _ => todo!("Error handling."),
        }
    }

    fn parse_assignment(&mut self, lhs: &mut Expr) -> Result<(), LoxError> {
        self.expect(tt!("="))?;
        let rhs = self.parse_expr()?;

        if let Expr::Variable(variable) = lhs {
            *lhs = Assign {
                variable: variable.name.clone(),
                value: Box::new(rhs),
            }
            .into();

            Ok(())
        } else {
            todo!("Error handling.")
        }
    }
}

impl ErrorReporter<ASTError> for ExprASTParser<'_, '_> {
    fn line(&self) -> usize {
        self.lexer.line()
    }
}
