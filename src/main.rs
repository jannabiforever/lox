use clap::Parser;
use codecrafters_interpreter::lox_tokenize;
use std::{fs, io, path::PathBuf, process::ExitCode};

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
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    match arg {
        Cli::Tokenize { file_name } => {
            let src = read(file_name);
            lox_tokenize(&src, &mut stdout, &mut stderr)
        }
    }
}
