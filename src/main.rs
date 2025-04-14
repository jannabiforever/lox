use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use clap::Subcommand;
use codecrafters_interpreter::lox_evaluate;
use codecrafters_interpreter::lox_parse;
use codecrafters_interpreter::lox_run;
use codecrafters_interpreter::lox_tokenize;

#[non_exhaustive]
#[derive(Debug, Parser)]
struct Cli {
    /// The command to run
    #[clap(subcommand)]
    command: LoxCommand,
}

#[derive(Debug, Subcommand)]
enum LoxCommand {
    Tokenize { file_name: PathBuf },
    Parse { file_name: PathBuf },
    Evaluate { file_name: PathBuf },
    Run { file_name: PathBuf },
}

fn read(file_name: PathBuf) -> String {
    fs::read_to_string(file_name).unwrap_or_default()
}

fn main() -> ExitCode {
    let arg = Cli::parse();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    match arg.command {
        LoxCommand::Tokenize { file_name } => {
            let src = read(file_name);
            lox_tokenize(&src, &mut stdout, &mut stderr)
        }
        LoxCommand::Parse { file_name } => {
            let src = read(file_name);
            lox_parse(&src, &mut stdout, &mut stderr)
        }
        LoxCommand::Evaluate { file_name } => {
            let src = read(file_name);
            lox_evaluate(&src, &mut stdout, &mut stderr)
        }
        LoxCommand::Run { file_name } => {
            let src = read(file_name);
            lox_run(&src, &mut stdout, &mut stderr)
        }
    }
}
