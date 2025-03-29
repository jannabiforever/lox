use clap::{Parser, Subcommand};
use codecrafters_interpreter::{lox_evaluate, lox_parse, lox_tokenize};
use std::{fs, io, path::PathBuf, process::ExitCode};

#[non_exhaustive]
#[derive(Debug, Parser)]
struct Cli {
    /// Debug mode
    #[arg(short, long)]
    debug: bool,

    /// The command to run
    #[clap(subcommand)]
    command: LoxCommand,
}

#[derive(Debug, Subcommand)]
enum LoxCommand {
    Tokenize { file_name: PathBuf },
    Parse { file_name: PathBuf },
    Evaluate { file_name: PathBuf },
}

fn read(file_name: PathBuf) -> String {
    fs::read_to_string(file_name).unwrap_or_default()
}

fn main() -> ExitCode {
    let arg = Cli::parse();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let debug = arg.debug;
    match arg.command {
        LoxCommand::Tokenize { file_name } => {
            let src = read(file_name);
            lox_tokenize(&src, &mut stdout, &mut stderr, debug)
        }
        LoxCommand::Parse { file_name } => {
            let src = read(file_name);
            lox_parse(&src, &mut stdout, &mut stderr, debug)
        }
        LoxCommand::Evaluate { file_name } => {
            let src = read(file_name);
            lox_evaluate(&src, &mut stdout, &mut stderr, debug)
        }
    }
}
