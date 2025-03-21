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
    std::fs::read_to_string(file_path).unwrap_or(String::new())
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
                let (line, inner) = token.into();
                match inner {
                    Ok(token) => println!("{}", token),
                    Err(error) => {
                        exit_code = error.exit_code();
                        eprintln!("{}: {}", line, error)
                    }
                }
            }
        }
        Command::Parse { file_path } => {
            todo!("Implement parsing.")
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
