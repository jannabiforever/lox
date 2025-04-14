use crate::expr::ExprParseError;
use crate::literal::Literal;
use crate::literal::Number;
use crate::mac::tt;

impl super::ExprParser<'_, '_> {
    pub(super) fn parse_literal(&mut self) -> Option<Result<Literal, ExprParseError>> {
        let peeked = self.token_stream.peek();
        let src = peeked.src;
        match peeked.token_type {
            tt!("nil") => {
                self.token_stream.next();
                Some(Ok(Literal::Nil))
            }
            tt!("true") => {
                self.token_stream.next();
                Some(Ok(Literal::Boolean(true)))
            }
            tt!("false") => {
                self.token_stream.next();
                Some(Ok(Literal::Boolean(false)))
            }
            tt!("number") => {
                self.token_stream.next();
                let num = src.parse::<Number>().unwrap();
                Some(Ok(Literal::Number(num)))
            }
            tt!("string") => {
                self.token_stream.next();
                let src = src.trim_matches('"');
                Some(Ok(Literal::String(src.to_string())))
            }
            _ => None,
        }
    }
}
