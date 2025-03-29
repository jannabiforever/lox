use crate::{
    literal::{Literal, Number},
    parse::ParseError,
    tokenize::tt,
};

impl super::ExprParser<'_, '_> {
    pub(super) fn parse_literal(&mut self) -> Option<Result<Literal, ParseError>> {
        let peeked = self.peek();
        let src = peeked.src;
        match peeked.token_type {
            tt!("nil") => {
                self.next();
                Some(Ok(Literal::Nil))
            }
            tt!("true") => {
                self.next();
                Some(Ok(Literal::Boolean(true)))
            }
            tt!("false") => {
                self.next();
                Some(Ok(Literal::Boolean(false)))
            }
            tt!("number") => {
                self.next();
                let num = src.parse::<Number>().unwrap();
                Some(Ok(Literal::Number(num)))
            }
            tt!("string") => {
                self.next();
                let src = src.trim_matches('"');
                Some(Ok(Literal::String(src.to_string())))
            }
            _ => None,
        }
    }

    pub(super) fn try_parse_variable(&mut self) -> Option<String> {
        let peeked = self.peek();
        let src = peeked.src;
        match peeked.token_type {
            tt!("identifier") => {
                self.next();
                Some(src.to_string())
            }
            _ => None,
        }
    }
}
