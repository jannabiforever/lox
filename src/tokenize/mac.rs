macro_rules! tt {
    ("") => {
        $crate::tokenize::TokenType::Eof
    };
    ("(") => {
        $crate::tokenize::TokenType::LeftParen
    };
    (")") => {
        $crate::tokenize::TokenType::RightParen
    };
    ("{") => {
        $crate::tokenize::TokenType::LeftBrace
    };
    ("}") => {
        $crate::tokenize::TokenType::RightBrace
    };
    (",") => {
        $crate::tokenize::TokenType::Comma
    };
    (".") => {
        $crate::tokenize::TokenType::Dot
    };
    ("-") => {
        $crate::tokenize::TokenType::Minus
    };
    ("+") => {
        $crate::tokenize::TokenType::Plus
    };
    (";") => {
        $crate::tokenize::TokenType::Semicolon
    };
    ("/") => {
        $crate::tokenize::TokenType::Slash
    };
    ("*") => {
        $crate::tokenize::TokenType::Star
    };
    ("!") => {
        $crate::tokenize::TokenType::Bang
    };
    ("!=") => {
        $crate::tokenize::TokenType::BangEqual
    };
    ("=") => {
        $crate::tokenize::TokenType::Equal
    };
    ("==") => {
        $crate::tokenize::TokenType::EqualEqual
    };
    ("<") => {
        $crate::tokenize::TokenType::Less
    };
    ("<=") => {
        $crate::tokenize::TokenType::LessEqual
    };
    (">") => {
        $crate::tokenize::TokenType::Greater
    };
    (">=") => {
        $crate::tokenize::TokenType::GreaterEqual
    };
    ("identifier") => {
        $crate::tokenize::TokenType::Identifier
    };
    ("string") => {
        $crate::tokenize::TokenType::String
    };
    ("number") => {
        $crate::tokenize::TokenType::Number
    };
    ("and") => {
        $crate::tokenize::TokenType::And
    };
    ("class") => {
        $crate::tokenize::TokenType::Class
    };
    ("else") => {
        $crate::tokenize::TokenType::Else
    };
    ("false") => {
        $crate::tokenize::TokenType::False
    };
    ("for") => {
        $crate::tokenize::TokenType::For
    };
    ("fun") => {
        $crate::tokenize::TokenType::Fun
    };
    ("if") => {
        $crate::tokenize::TokenType::If
    };
    ("nil") => {
        $crate::tokenize::TokenType::Nil
    };
    ("or") => {
        $crate::tokenize::TokenType::Or
    };
    ("print") => {
        $crate::tokenize::TokenType::Print
    };
    ("return") => {
        $crate::tokenize::TokenType::Return
    };
    ("super") => {
        $crate::tokenize::TokenType::Super
    };
    ("this") => {
        $crate::tokenize::TokenType::This
    };
    ("true") => {
        $crate::tokenize::TokenType::True
    };
    ("var") => {
        $crate::tokenize::TokenType::Var
    };
    ("while") => {
        $crate::tokenize::TokenType::While
    };
}

pub(crate) use tt;
