/// Wrapper for Rc::new(RefCell::new(~))
macro_rules! rc_rc {
    ($inner:expr) => {
        std::rc::Rc::new(std::cell::RefCell::new($inner))
    };
}

/// Implement From for wrapper enums.
macro_rules! impl_from {
    ( $target:ident : $( $variant:ident ),* ) => {
        $(
            impl From<$variant> for $target {
                fn from(value: $variant) -> Self {
                    Self::$variant(value)
                }
            }
        )*
    };
}

macro_rules! tt {
    ("") => {
        $crate::token::TokenType::Eof
    };
    ("(") => {
        $crate::token::TokenType::LeftParen
    };
    (")") => {
        $crate::token::TokenType::RightParen
    };
    ("{") => {
        $crate::token::TokenType::LeftBrace
    };
    ("}") => {
        $crate::token::TokenType::RightBrace
    };
    (",") => {
        $crate::token::TokenType::Comma
    };
    (".") => {
        $crate::token::TokenType::Dot
    };
    ("-") => {
        $crate::token::TokenType::Minus
    };
    ("+") => {
        $crate::token::TokenType::Plus
    };
    (";") => {
        $crate::token::TokenType::Semicolon
    };
    ("/") => {
        $crate::token::TokenType::Slash
    };
    ("*") => {
        $crate::token::TokenType::Star
    };
    ("!") => {
        $crate::token::TokenType::Bang
    };
    ("!=") => {
        $crate::token::TokenType::BangEqual
    };
    ("=") => {
        $crate::token::TokenType::Equal
    };
    ("==") => {
        $crate::token::TokenType::EqualEqual
    };
    ("<") => {
        $crate::token::TokenType::Less
    };
    ("<=") => {
        $crate::token::TokenType::LessEqual
    };
    (">") => {
        $crate::token::TokenType::Greater
    };
    (">=") => {
        $crate::token::TokenType::GreaterEqual
    };
    ("identifier") => {
        $crate::token::TokenType::Identifier
    };
    ("string") => {
        $crate::token::TokenType::String
    };
    ("number") => {
        $crate::token::TokenType::Number
    };
    ("and") => {
        $crate::token::TokenType::And
    };
    ("class") => {
        $crate::token::TokenType::Class
    };
    ("else") => {
        $crate::token::TokenType::Else
    };
    ("false") => {
        $crate::token::TokenType::False
    };
    ("for") => {
        $crate::token::TokenType::For
    };
    ("fun") => {
        $crate::token::TokenType::Fun
    };
    ("if") => {
        $crate::token::TokenType::If
    };
    ("nil") => {
        $crate::token::TokenType::Nil
    };
    ("or") => {
        $crate::token::TokenType::Or
    };
    ("print") => {
        $crate::token::TokenType::Print
    };
    ("return") => {
        $crate::token::TokenType::Return
    };
    ("super") => {
        $crate::token::TokenType::Super
    };
    ("this") => {
        $crate::token::TokenType::This
    };
    ("true") => {
        $crate::token::TokenType::True
    };
    ("var") => {
        $crate::token::TokenType::Var
    };
    ("while") => {
        $crate::token::TokenType::While
    };
}

pub(crate) use impl_from;
pub(crate) use rc_rc;
pub(crate) use tt;
