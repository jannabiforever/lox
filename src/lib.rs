mod literal;
mod tokenize;

use std::{io::Write, process::ExitCode};

/// Entry point for 'tokenize' command.
pub fn lox_tokenize<W1, W2>(src: &str, ok_buf: &mut W1, _: &mut W2) -> ExitCode
where
    W1: Write,
    W2: Write,
{
    let tokens = tokenize::Tokenizer::new(src).tokenize();
    for token in tokens {
        writeln!(ok_buf, "{}", token).unwrap();
    }
    ExitCode::SUCCESS
}
