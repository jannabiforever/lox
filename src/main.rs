use clap::Parser;
use codecrafters_interpreter::lox_tokenize;
use std::{fs, path::PathBuf, process::ExitCode};

#[non_exhaustive]
#[derive(Debug, Parser)]
enum Cli {
    Tokenize { file_name: PathBuf },
}

fn read(file_name: PathBuf) -> String {
    fs::read_to_string(file_name).unwrap_or_default()
}

fn main() -> ExitCode {
    let arg = Cli::parse();
    #[allow(unused_mut)]
    let mut exit_code = ExitCode::SUCCESS;

    match arg {
        Cli::Tokenize { file_name } => {
            let src = read(file_name);
            lox_tokenize(&src);
        }
    }

    exit_code
}
