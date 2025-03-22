//! Parses the AST from tokens.
mod unit;

use self::unit::{Binary, BinaryOp, Expr, Grouping, Unary, UnaryOp};
use crate::{
    error::{ASTError, ErrorReporter, LoxError, WithLine},
    lex::{Token, TokenType, lexer::Lexer, tt},
    value::Literal,
};

pub fn generate_expr_ast(source: &str) -> WithLine<Result<Expr, LoxError>> {
    let lexer = Lexer::new(source);
    let mut generator = ExprASTParser::new(lexer);
    generator.parse_expr_with_line()
}

struct ExprASTParser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> ExprASTParser<'a> {
    pub fn next(&mut self) -> Result<Token<'a>, LoxError> {
        let token = self.lexer.scan_token()?;
        Ok(token)
    }

    pub fn peek(&self) -> Result<Token<'a>, LoxError> {
        let token = self.lexer.clone().scan_token()?;
        Ok(token)
    }
}

impl<'a> ExprASTParser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse_expr_with_line(&mut self) -> WithLine<Result<Expr, LoxError>> {
        let result = self.parse_expr();
        self.wrap(result)
    }

    pub fn parse_expr(&mut self) -> Result<Expr, LoxError> {
        let mut lhs = self.parse_expr_unit()?;

        loop {
            let peeked_token = self.peek()?;
            match &peeked_token.token_type {
                // End of expression, so don't consume the token.
                tt!("") | tt!(";") | tt!(")") | tt!("}") | tt!(",") => {
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
                | tt!("<=") => {
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
        Ok(if let Expr::Binary(binary) = lhs {
            // Compare the precedence of the current operator with the operator in the binary expression.
            binary.bind(new_op, rhs);
        } else {
            *lhs = Binary {
                left: Box::new(lhs.clone()),
                op: new_op,
                right: Box::new(rhs),
            }
            .into();
        })
    }

    /// Consumes the first ahead expression unit.
    fn parse_expr_unit(&mut self) -> Result<Expr, LoxError> {
        let cur_token = self.next()?;

        let source = cur_token.source;
        match &cur_token.token_type {
            // Literal expression.
            TokenType::Number => Ok(Literal::number_from_source(source.to_string()).into()),
            TokenType::String => Ok(Literal::string_from_source(source.to_string()).into()),
            TokenType::True => Ok(Literal::Boolean(true).into()),
            TokenType::False => Ok(Literal::Boolean(false).into()),
            TokenType::Nil => Ok(Literal::Nil.into()),
            // Grouping expression.
            TokenType::LeftParen => {
                let inner_expr = self.parse_expr()?;
                self.expect_next_token_to_be(TokenType::RightParen)?;

                let grouping_expr = Grouping {
                    inner: Box::new(inner_expr),
                };

                Ok(grouping_expr.into())
            }
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

    /// Consumes the next token and checks if it is the expected token.
    fn expect_next_token_to_be(&mut self, expected: TokenType) -> Result<Token, LoxError> {
        let next_token = self.lexer.scan_token()?;
        if next_token.token_type == expected {
            Ok(next_token)
        } else {
            todo!("Error handling.")
        }
    }
}

impl ErrorReporter<ASTError> for ExprASTParser<'_> {
    fn line(&self) -> usize {
        self.lexer.line()
    }
}
