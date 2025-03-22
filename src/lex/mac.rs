macro_rules! tt {
    ("") => {
        $crate::lex::TokenType::Eof
    };
    ("(") => {
        $crate::lex::TokenType::LeftParen
    };
    (")") => {
        $crate::lex::TokenType::RightParen
    };
    ("{") => {
        $crate::lex::TokenType::LeftBrace
    };
    ("}") => {
        $crate::lex::TokenType::RightBrace
    };
    (",") => {
        $crate::lex::TokenType::Comma
    };
    (".") => {
        $crate::lex::TokenType::Dot
    };
    ("-") => {
        $crate::lex::TokenType::Minus
    };
    ("+") => {
        $crate::lex::TokenType::Plus
    };
    (";") => {
        $crate::lex::TokenType::Semicolon
    };
    ("/") => {
        $crate::lex::TokenType::Slash
    };
    ("*") => {
        $crate::lex::TokenType::Star
    };
    ("!") => {
        $crate::lex::TokenType::Bang
    };
    ("!=") => {
        $crate::lex::TokenType::BangEqual
    };
    ("=") => {
        $crate::lex::TokenType::Equal
    };
    ("==") => {
        $crate::lex::TokenType::EqualEqual
    };
    ("<") => {
        $crate::lex::TokenType::Less
    };
    ("<=") => {
        $crate::lex::TokenType::LessEqual
    };
    (">") => {
        $crate::lex::TokenType::Greater
    };
    (">=") => {
        $crate::lex::TokenType::GreaterEqual
    };
    ("identifier") => {
        $crate::lex::TokenType::Identifier
    };
    ("string") => {
        $crate::lex::TokenType::String
    };
    ("number") => {
        $crate::lex::TokenType::Number
    };
    ("and") => {
        $crate::lex::TokenType::And
    };
    ("class") => {
        $crate::lex::TokenType::Class
    };
    ("else") => {
        $crate::lex::TokenType::Else
    };
    ("false") => {
        $crate::lex::TokenType::False
    };
    ("for") => {
        $crate::lex::TokenType::For
    };
    ("fun") => {
        $crate::lex::TokenType::Fun
    };
    ("if") => {
        $crate::lex::TokenType::If
    };
    ("nil") => {
        $crate::lex::TokenType::Nil
    };
    ("or") => {
        $crate::lex::TokenType::Or
    };
    ("print") => {
        $crate::lex::TokenType::Print
    };
    ("return") => {
        $crate::lex::TokenType::Return
    };
    ("super") => {
        $crate::lex::TokenType::Super
    };
    ("this") => {
        $crate::lex::TokenType::This
    };
    ("true") => {
        $crate::lex::TokenType::True
    };
    ("var") => {
        $crate::lex::TokenType::Var
    };
    ("while") => {
        $crate::lex::TokenType::While
    };
}

pub(crate) use tt;
