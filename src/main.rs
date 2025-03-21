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

fn main() -> ExitCode {
    let command = Command::parse();
    #[allow(unused_variables)]
    match command {
        Command::Tokenize { file_path } => ExitCode::SUCCESS,
        Command::Parse { file_path } => ExitCode::SUCCESS,
        Command::Evaluate { file_path } => ExitCode::SUCCESS,
        Command::Run { file_path } => ExitCode::SUCCESS,
    }
}
