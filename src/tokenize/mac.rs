macro_rules! tt {
    ("") => {
        $crate::tokenize::token::TokenType::Eof
    };
    ("(") => {
        $crate::tokenize::token::TokenType::LeftParen
    };
    (")") => {
        $crate::tokenize::token::TokenType::RightParen
    };
    ("{") => {
        $crate::tokenize::token::TokenType::LeftBrace
    };
    ("}") => {
        $crate::tokenize::token::TokenType::RightBrace
    };
    (",") => {
        $crate::tokenize::token::TokenType::Comma
    };
    (".") => {
        $crate::tokenize::token::TokenType::Dot
    };
    ("-") => {
        $crate::tokenize::token::TokenType::Minus
    };
    ("+") => {
        $crate::tokenize::token::TokenType::Plus
    };
    (";") => {
        $crate::tokenize::token::TokenType::Semicolon
    };
    ("/") => {
        $crate::tokenize::token::TokenType::Slash
    };
    ("*") => {
        $crate::tokenize::token::TokenType::Star
    };
    ("!") => {
        $crate::tokenize::token::TokenType::Bang
    };
    ("!=") => {
        $crate::tokenize::token::TokenType::BangEqual
    };
    ("=") => {
        $crate::tokenize::token::TokenType::Equal
    };
    ("==") => {
        $crate::tokenize::token::TokenType::EqualEqual
    };
    ("<") => {
        $crate::tokenize::token::TokenType::Less
    };
    ("<=") => {
        $crate::tokenize::token::TokenType::LessEqual
    };
    (">") => {
        $crate::tokenize::token::TokenType::Greater
    };
    (">=") => {
        $crate::tokenize::token::TokenType::GreaterEqual
    };
    ("identifier") => {
        $crate::tokenize::token::TokenType::Identifier
    };
    ("string") => {
        $crate::tokenize::token::TokenType::String
    };
    ("number") => {
        $crate::tokenize::token::TokenType::Number
    };
    ("and") => {
        $crate::tokenize::token::TokenType::And
    };
    ("class") => {
        $crate::tokenize::token::TokenType::Class
    };
    ("else") => {
        $crate::tokenize::token::TokenType::Else
    };
    ("false") => {
        $crate::tokenize::token::TokenType::False
    };
    ("for") => {
        $crate::tokenize::token::TokenType::For
    };
    ("fun") => {
        $crate::tokenize::token::TokenType::Fun
    };
    ("if") => {
        $crate::tokenize::token::TokenType::If
    };
    ("nil") => {
        $crate::tokenize::token::TokenType::Nil
    };
    ("or") => {
        $crate::tokenize::token::TokenType::Or
    };
    ("print") => {
        $crate::tokenize::token::TokenType::Print
    };
    ("return") => {
        $crate::tokenize::token::TokenType::Return
    };
    ("super") => {
        $crate::tokenize::token::TokenType::Super
    };
    ("this") => {
        $crate::tokenize::token::TokenType::This
    };
    ("true") => {
        $crate::tokenize::token::TokenType::True
    };
    ("var") => {
        $crate::tokenize::token::TokenType::Var
    };
    ("while") => {
        $crate::tokenize::token::TokenType::While
    };
}

pub(crate) use tt;
