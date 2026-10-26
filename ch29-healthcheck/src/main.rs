//! Chapter 29: Project 1 — A Health Check CLI Tool

use ch29_healthcheck::{check_all, checker, load_targets, mock::MockServer, CheckResult, Config};
use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

/// Check a list of HTTP endpoints and report which are healthy.
#[derive(Debug, Parser)]
#[command(name = "healthcheck", version, about)]
struct Args {
    /// File with one URL per line, or `-` for stdin
    file: Option<PathBuf>,

    /// Per-request timeout in seconds
    #[arg(short, long, default_value_t = 3)]
    timeout: u64,

    /// Concurrent workers
    #[arg(short, long, default_value_t = 8)]
    workers: usize,

    /// Emit JSON instead of a table
    #[arg(long)]
    json: bool,

    /// Start a local server with known-good and known-bad targets
    #[arg(long)]
    selftest: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    match run(&args) {
        // All healthy: 0. Ran but something is down: 1. Could not run: 2.
        Ok(results) if results.iter().all(CheckResult::healthy) => ExitCode::SUCCESS,
        Ok(_) => ExitCode::from(1),
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}

fn run(args: &Args) -> Result<Vec<CheckResult>, String> {
    let cfg = Config {
        timeout: Duration::from_secs(args.timeout),
        workers: args.workers,
    };

    let (targets, _server) = if args.selftest {
        let server = MockServer::spawn();
        let targets = vec![
            server.url("/health"),
            server.url("/slow"),
            server.url("/down"),
            server.url("/missing"),
        ];
        (targets, Some(server))
    } else {
        let targets = match args.file.as_deref() {
            Some(p) if p.as_os_str() == "-" => {
                let mut buf = String::new();
                std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf)
                    .map_err(|e| e.to_string())?;
                buf.lines()
                    .map(str::trim)
                    .filter(|l| !l.is_empty() && !l.starts_with('#'))
                    .map(str::to_string)
                    .collect()
            }
            Some(p) => load_targets(p)?,
            None => {
                return Err("no targets: pass a file, `-` for stdin, or --selftest".into());
            }
        };
        (targets, None)
    };

    if targets.is_empty() {
        return Err("no targets found".into());
    }

    let results = check_all(&targets, &cfg);

    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&results).map_err(|e| e.to_string())?
        );
    } else {
        checker::print_human(&results);
    }

    Ok(results)
}
