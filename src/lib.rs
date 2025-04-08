mod env;
mod error;
mod evaluate;
mod literal;
mod mac;
mod parse;
mod run;
mod tokenize;

use std::{cell::RefCell, io::Write, process::ExitCode, rc::Rc};

use mac::rc_rc;

use self::error::LoxError;
use self::error::LoxErrorKind;
use self::tokenize::TokenStream;

/// tokenize without allowing error.
macro_rules! tokenize {
    ($src:expr, $err_buf:ident) => {
        match tokenize::Tokenizer::new($src)
            .tokenize()
            .into_iter()
            .collect::<Result<Vec<_>, LoxError>>()
        {
            Ok(tokens) => tokens,
            Err(err) => {
                writeln!($err_buf, "{err}").unwrap();
                return ExitCode::from(65);
            }
        }
    };
}

/// Entry point for 'tokenize' command.
pub fn lox_tokenize<W1, W2>(src: &str, ok_buf: &mut W1, err_buf: &mut W2) -> ExitCode
where
    W1: Write,
    W2: Write,
{
    let tokens = tokenize::Tokenizer::new(src).tokenize();
    let mut exit_code = ExitCode::SUCCESS;

    for token in tokens {
        match token.as_ref() {
            Ok(token) => writeln!(ok_buf, "{token}").unwrap(),
            Err(error_message) => {
                writeln!(err_buf, "{error_message}").unwrap();
                exit_code = ExitCode::from(65);
            }
        }
    }

    exit_code
}

/// Entry point for 'parse' command.
pub fn lox_parse<W1, W2>(src: &str, ok_buf: &mut W1, err_buf: &mut W2) -> ExitCode
where
    W1: Write,
    W2: Write,
{
    // Try tokenizing. If fails, don't parse.
    let tokens = tokenize!(src, err_buf);

    let mut stream = TokenStream::new(&tokens);
    let parsed = parse::ExprParser::new(&mut stream).parse_with_line();

    match parsed {
        Ok(ast) => {
            writeln!(ok_buf, "{ast}").unwrap();
            ExitCode::SUCCESS
        }
        Err(err) => {
            writeln!(err_buf, "{err}").unwrap();
            ExitCode::from(65)
        }
    }
}

/// Entry point for 'evaluate' command.
pub fn lox_evaluate<W1, W2>(src: &str, ok_buf: &mut W1, err_buf: &mut W2) -> ExitCode
where
    W1: Write,
    W2: Write,
{
    let tokens = tokenize!(src, err_buf);

    let mut stream = TokenStream::new(&tokens);
    let parsed = match parse::ExprParser::new(&mut stream).parse_with_line() {
        Ok(ast) => ast,
        Err(err) => {
            writeln!(err_buf, "{err}").unwrap();
            return ExitCode::from(65);
        }
    };

    let evaluator = evaluate::Evaluator::new();
    let result = evaluator.eval(&parsed);

    // Pretty print for evaluate command.
    let result = result.map(|res| res.pretty());

    match result {
        Ok(result) => writeln!(ok_buf, "{result}").unwrap(),
        Err(error_message) => {
            writeln!(err_buf, "{error_message}").unwrap();
            return ExitCode::from(70);
        }
    }

    ExitCode::SUCCESS
}

/// Entry point for 'run' command.
pub fn lox_run<W1, W2>(src: &str, ok_buf: &mut W1, err_buf: &mut W2) -> ExitCode
where
    W1: Write,
    W2: Write,
{
    let tokens = tokenize!(src, err_buf);

    let mut stream = TokenStream::new(&tokens);
    let stmts = match run::StmtParser::new(&mut stream).parse_all() {
        Ok(stmts) => stmts,
        Err(err) => {
            writeln!(err_buf, "{err}").unwrap();
            return ExitCode::from(65);
        }
    };

    let runtime = run::Runtime::new(rc_rc!(ok_buf));
    for stmt in stmts {
        if let Err(err) = runtime.run(stmt) {
            writeln!(err_buf, "{err}").unwrap();
            return ExitCode::from(70);
        }
    }

    ExitCode::SUCCESS
}
