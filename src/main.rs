use clap::Parser;
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, clap::Parser)]
pub enum Command {
    /// Tokenize the input source code.
    Tokenize { file_path: PathBuf },
    /// Parse the input source code into an AST.
    Parse { file_path: PathBuf },
    /// Evaluate the input expression.
    Evaluate { file_path: PathBuf },
    /// Compile and run the input source code.
    Run { file_path: PathBuf },
}

fn file_contents(file_path: PathBuf) -> String {
    std::fs::read_to_string(file_path).unwrap_or_default()
}

fn main() -> ExitCode {
    let command = Command::parse();

    let mut exit_code = ExitCode::SUCCESS;

    #[allow(unused_variables)]
    match command {
        Command::Tokenize { file_path } => {
            let file_contents = file_contents(file_path);
            let tokens = lox::lex::scan(&file_contents);

            for token in tokens {
                match token.cast_down() {
                    Ok(token) => println!("{}", token),
                    Err(error) => {
                        eprintln!("{}", error);
                        exit_code = error.into_inner().exit_code();
                    }
                }
            }
        }
        Command::Parse { file_path } => {
            let file_contents = file_contents(file_path);
            let ast_result = lox::expr_ast::parse_expr_ast(&file_contents).cast_down();

            match ast_result {
                Ok(ast) => println!("{}", ast),
                Err(error) => {
                    eprintln!("{}", error);
                    exit_code = error.into_inner().exit_code();
                }
            }
        }
        Command::Evaluate { file_path } => {
            let file_contents = file_contents(file_path);
            let value = lox::exe::evaluate_single_expr_ast(&file_contents);

            match value {
                Ok(value) => println!("{}", value),
                Err(error) => {
                    eprintln!("{}", error);
                    exit_code = error.into_inner().exit_code();
                }
            }
        }
        Command::Run { file_path } => {
            let file_contents = file_contents(file_path);
            let result = lox::exe::execute_stmt_ast(&file_contents);
            if let Err(e) = result {
                eprintln!("{}", e);
                exit_code = e.into_inner().exit_code();
            }
        }
    }

    exit_code
}
