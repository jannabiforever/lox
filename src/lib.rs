mod env;
mod error;
mod expr;
mod literal;
mod mac;
mod statement;
mod token;

use std::{cell::RefCell, io::Write, process::ExitCode, rc::Rc};

use env::{Environment, Evaluatable};
use error::LoxResulT;

use self::error::IntoLoxError;
use self::mac::rc_rc;
use self::token::TokenStream;

/// tokenize without allowing error.
macro_rules! tokenize {
    ($src:expr, $err_buf:ident) => {
        match token::Tokenizer::new($src)
            .tokenize()
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(tokens) => tokens,
            Err(err) => {
                writeln!($err_buf, "{err}").unwrap();
                return err.kind.exit_code();
            }
        }
    };
}

/// parse expression without allowing error.
macro_rules! expr_parse {
    ($stream:expr, $err_buf:ident) => {
        match expr::ExprParser::new(&mut $stream).parse_with_line() {
            Ok(ast) => ast,
            Err(err) => {
                writeln!($err_buf, "{err}").unwrap();
                return err.kind.exit_code();
            }
        }
    };
}

/// parse statements without allowing error.
macro_rules! stmt_parse {
    ($stream:expr, $err_buf:ident) => {
        match statement::StmtParser::new(&mut $stream).parse_all() {
            Ok(stmts) => stmts,
            Err(err) => {
                writeln!($err_buf, "{err}").unwrap();
                return err.kind.exit_code();
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
    let mut exit_code = ExitCode::SUCCESS;
    let tokens = token::Tokenizer::new(src).tokenize();

    for token in tokens {
        if let Err(e) = token.write_to_buffer(ok_buf, err_buf) {
            exit_code = e;
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

    if let Err(exit_code) = expr::ExprParser::new(&mut stream)
        .parse_with_line()
        .write_to_buffer(ok_buf, err_buf)
    {
        exit_code
    } else {
        ExitCode::SUCCESS
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
    let parsed = expr_parse!(stream, err_buf);

    match parsed.eval(Environment::new()).map(|res| res.pretty()) {
        Ok(result) => writeln!(ok_buf, "{result}").unwrap(),
        Err(err) => {
            writeln!(err_buf, "{err}").unwrap();
            return err.exit_code();
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
    let stmts = stmt_parse!(stream, err_buf);

    let runtime = statement::Runtime::new(rc_rc!(ok_buf));
    for stmt in stmts {
        if let Err(err) = runtime.run(stmt) {
            writeln!(err_buf, "{err}").unwrap();
            return err.exit_code();
        }
    }

    ExitCode::SUCCESS
}
