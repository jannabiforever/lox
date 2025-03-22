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
                match token.into() {
                    Ok(token) => println!("{}", token),
                    Err(error) => {
                        exit_code = error.as_ref().exit_code();
                        eprintln!("{}", error)
                    }
                }
            }
        }
        Command::Parse { file_path } => {
            let file_contents = file_contents(file_path);
            let ast_result = lox::expr_ast::generate_expr_ast(&file_contents).into();

            match ast_result {
                Ok(ast) => println!("{}", ast),
                Err(error) => {
                    exit_code = error.as_ref().exit_code();
                    eprintln!("{}", error)
                }
            }
        }
        Command::Evaluate { file_path } => {
            todo!("Implement evaluation.")
        }
        Command::Run { file_path } => {
            todo!("Implement running.")
        }
    }

    exit_code
}
