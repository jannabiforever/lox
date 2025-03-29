mod error;
mod evaluate;
mod literal;
mod parse;
mod tokenize;

use std::{io::Write, process::ExitCode};

use self::error::{LoxError, WithLine};

/// Entry point for 'tokenize' command.
pub fn lox_tokenize<W1, W2>(src: &str, ok_buf: &mut W1, err_buf: &mut W2, debug: bool) -> ExitCode
where
    W1: Write,
    W2: Write,
{
    let tokens = tokenize::Tokenizer::new(src).tokenize();
    let mut exit_code = ExitCode::SUCCESS;
    if debug {
        for token in tokens {
            match token.exposure() {
                Ok(token) => writeln!(ok_buf, "{:?}", token).unwrap(),
                Err(error_message) => {
                    writeln!(err_buf, "{:?}", error_message).unwrap();
                    exit_code = ExitCode::from(65);
                }
            }
        }
    } else {
        for token in tokens {
            match token.exposure() {
                Ok(token) => writeln!(ok_buf, "{}", token).unwrap(),
                Err(error_message) => {
                    writeln!(err_buf, "{}", error_message).unwrap();
                    exit_code = ExitCode::from(65);
                }
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
        .map(|t_res| t_res.cast_down())
        .collect::<Result<Vec<_>, WithLine<LoxError>>>()
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

    let parsed = parse::ExprParser::new(&tokens).parse_with_line();
    if debug {
        match parsed.exposure() {
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
        match parsed.exposure() {
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
        .map(|t_res| t_res.cast_down())
        .collect::<Result<Vec<_>, WithLine<LoxError>>>()
    {
        Ok(tokens) => tokens,
        Err(err) => {
            writeln!(err_buf, "{}", err).unwrap();
            return ExitCode::from(65);
        }
    };

    let parsed = match parse::ExprParser::new(&tokens)
        .parse_with_line()
        .cast_down()
    {
        Ok(ast_with_line) => ast_with_line.into_inner(),
        Err(err) => {
            if debug {
                writeln!(err_buf, "{:?}", err).unwrap();
            } else {
                writeln!(err_buf, "{}", err).unwrap();
            }
            return ExitCode::from(65);
        }
    };

    let evaluator = evaluate::Evaluator;
    let result = evaluator.eval(&parsed);

    if debug {
        writeln!(ok_buf, "{:?}", result).unwrap();
    } else {
        writeln!(ok_buf, "{}", result).unwrap();
    }

    ExitCode::SUCCESS
}
