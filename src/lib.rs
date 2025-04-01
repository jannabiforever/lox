mod error;
mod evaluate;
mod literal;
mod parse;
mod tokenize;

use std::{io::Write, process::ExitCode};

use error::LoxError;
use literal::Number;
use tokenize::TokenStream;

use self::error::LoxErrorKind;

macro_rules! debug_writeln {
    ($w:expr, $obj:ident, $debug:ident) => {
        if $debug {
            writeln!($w, "{:?}", $obj).unwrap();
        } else {
            writeln!($w, "{}", $obj).unwrap();
        }
    };
}

/// Entry point for 'tokenize' command.
pub fn lox_tokenize<W1, W2>(src: &str, ok_buf: &mut W1, err_buf: &mut W2, debug: bool) -> ExitCode
where
    W1: Write,
    W2: Write,
{
    let tokens = tokenize::Tokenizer::new(src).tokenize();
    let mut exit_code = ExitCode::SUCCESS;

    for token in tokens {
        match token.as_ref() {
            Ok(token) => debug_writeln!(ok_buf, token, debug),
            Err(error_message) => {
                debug_writeln!(err_buf, error_message, debug);
                exit_code = ExitCode::from(65);
            }
        }
    }

    exit_code
}

/// Entry point for 'parse' command.
pub fn lox_parse<W1, W2>(src: &str, ok_buf: &mut W1, err_buf: &mut W2, debug: bool) -> ExitCode
where
    W1: Write,
    W2: Write,
{
    // Try tokenizing. If fails, don't parse.
    let tokens = match tokenize::Tokenizer::new(src)
        .tokenize()
        .into_iter()
        .collect::<Result<Vec<_>, LoxError>>()
    {
        Ok(tokens) => tokens,
        Err(err) => {
            if debug {
                writeln!(err_buf, "{:?}", err).unwrap();
            } else {
                writeln!(err_buf, "{}", err).unwrap();
            }
            return ExitCode::from(65);
        }
    };

    let mut stream = TokenStream::new(&tokens);
    let parsed = parse::ExprParser::new(&mut stream).parse_with_line();
    if debug {
        match parsed {
            Ok(ast) => {
                writeln!(ok_buf, "{:?}", ast).unwrap();
                ExitCode::SUCCESS
            }
            Err(err) => {
                writeln!(err_buf, "{:?}", err).unwrap();
                ExitCode::from(65)
            }
        }
    } else {
        match parsed {
            Ok(ast) => {
                writeln!(ok_buf, "{}", ast).unwrap();
                ExitCode::SUCCESS
            }
            Err(err) => {
                writeln!(err_buf, "{}", err).unwrap();
                ExitCode::from(65)
            }
        }
    }
}

/// Entry point for 'evaluate' command.
/// Currently not implemented.
pub fn lox_evaluate<W1, W2>(src: &str, ok_buf: &mut W1, err_buf: &mut W2, debug: bool) -> ExitCode
where
    W1: Write,
    W2: Write,
{
    let tokens = match tokenize::Tokenizer::new(src)
        .tokenize()
        .into_iter()
        .collect::<Result<Vec<_>, LoxError>>()
    {
        Ok(tokens) => tokens,
        Err(err) => {
            writeln!(err_buf, "{}", err).unwrap();
            return ExitCode::from(65);
        }
    };

    let mut stream = TokenStream::new(&tokens);
    let parsed = match parse::ExprParser::new(&mut stream).parse_with_line() {
        Ok(ast) => ast,
        Err(err) => {
            debug_writeln!(err_buf, err, debug);
            return ExitCode::from(65);
        }
    };

    let evaluator = evaluate::Evaluator;
    let result = evaluator.eval(&parsed);

    let result = result.map(|res| match res {
        literal::Literal::Number(Number(number)) => number.to_string(),
        res => res.to_string(),
    });

    match result {
        Ok(result) => debug_writeln!(ok_buf, result, debug),
        Err(error_message) => {
            debug_writeln!(err_buf, error_message, debug);
            return ExitCode::from(70);
        }
    }

    ExitCode::SUCCESS
}
