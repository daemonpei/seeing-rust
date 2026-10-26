//! Chapter 30: Project 2 — A JSON Configuration Validator

use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(name = "validate-config", version, about = "Validate a service JSON config")]
struct Args {
    /// The JSON file to check
    file: PathBuf,

    /// Machine-readable output
    #[arg(long)]
    json: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
    match ch30_config_validator::run(&args.file, args.json) {
        Ok(0) => ExitCode::SUCCESS,
        Ok(_) => ExitCode::from(1),
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}
