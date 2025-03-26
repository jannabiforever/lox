mod error;
mod literal;
mod parse;
mod tokenize;

use std::{io::Write, process::ExitCode};

/// Entry point for 'tokenize' command.
pub fn lox_tokenize<W1, W2>(src: &str, ok_buf: &mut W1, err_buf: &mut W2) -> ExitCode
where
    W1: Write,
    W2: Write,
{
    let tokens = tokenize::Tokenizer::new(src).tokenize();
    let mut exit_code = ExitCode::SUCCESS;
    for token in tokens {
        match token.exposure() {
            Ok(token) => writeln!(ok_buf, "{}", token).unwrap(),
            Err(error_message) => {
                writeln!(err_buf, "{}", error_message).unwrap();
                exit_code = ExitCode::from(65);
            }
        }
    }
    exit_code
}

/// Entry point for 'parse' command.
pub fn lox_parse<W1, W2>(_src: &str, _ok_buf: &mut W1, _err_buf: &mut W2) -> ExitCode
where
    W1: Write,
    W2: Write,
{
    unimplemented!()
}
