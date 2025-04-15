mod env;
mod error;
mod expr;
mod function;
mod literal;
mod mac;
mod statement;
mod token;

use std::{io::Write, process::ExitCode};

use self::{
    env::{Env, Evaluatable, Runnable},
    error::{IntoLoxError, LoxResult},
    mac::rc_rc,
    token::TokenStream,
};

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
pub fn lox_tokenize<W1: Write, W2: Write>(
    src: &str,
    ok_buf: &mut W1,
    err_buf: &mut W2,
) -> ExitCode {
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
pub fn lox_parse<W1: Write, W2: Write>(src: &str, ok_buf: &mut W1, err_buf: &mut W2) -> ExitCode {
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
pub fn lox_evaluate<W1: Write, W2: Write>(
    src: &str,
    ok_buf: &mut W1,
    err_buf: &mut W2,
) -> ExitCode {
    let tokens = tokenize!(src, err_buf);

    let mut stream = TokenStream::new(&tokens);
    let parsed = expr_parse!(stream, err_buf);

    // Since 'evaluate' command doesn't actually print anything while evaluating,
    // we can set env.stdout to be some blank buffer.
    let empty_env = Env::new(Vec::new());

    if let Err(exit_code) = parsed
        .eval_lox(empty_env)
        .map(|res| res.to_string())
        .write_to_buffer(ok_buf, err_buf)
    {
        exit_code
    } else {
        ExitCode::SUCCESS
    }
}

/// Entry point for 'run' command.
pub fn lox_run<W1: Write, W2: Write>(src: &str, ok_buf: &mut W1, err_buf: &mut W2) -> ExitCode {
    let tokens = tokenize!(src, err_buf);

    let mut stream = TokenStream::new(&tokens);
    let stmts = stmt_parse!(stream, err_buf);

    let env = Env::new(ok_buf);
    for stmt in stmts {
        if let Err(err) = stmt.run(env.clone()) {
            writeln!(err_buf, "{err}").unwrap();
            return err.exit_code();
        }
    }

    ExitCode::SUCCESS
}
