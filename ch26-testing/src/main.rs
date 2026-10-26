//! Chapter 26: Testing With Cargo
//!
//! A binary so `tests/integration.rs` has something to run with
//! `env!("CARGO_BIN_EXE_ch26")`.

use std::path::Path;

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("ch26 <config-file>");
        println!();
        println!("  Parses `service=` and `port=` lines and validates them.");
        println!("  Exits 0 when valid, 1 when invalid, 2 when the file is missing.");
        return std::process::ExitCode::SUCCESS;
    }

    let Some(path) = args.first() else {
        eprintln!("usage: ch26 <config-file>");
        return std::process::ExitCode::from(2);
    };

    match ch26_testing::Config::load(Path::new(path)) {
        Ok(cfg) => {
            println!("ok: {} on port {}", cfg.service, cfg.port);
            std::process::ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("invalid: {e}");
            std::process::ExitCode::from(1)
        }
    }
}
