//! Chapter 19: Command-Line Programs
//!
//! Run it with no arguments and it processes a generated sample log; pass a file
//! to filter that instead; pass `-` to read stdin.

use clap::Parser;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process::ExitCode;

/// Filter a log file by a substring.
#[derive(Debug, Parser)]
#[command(name = "logfilter", version, about)]
struct Args {
    /// File to read, or `-` for stdin. Defaults to a generated sample.
    file: Option<PathBuf>,

    /// Text to search for
    ///
    /// No short form here: `-n` is taken by `--line-numbers`. The book's listing
    /// gives this argument `short`, and clap panics at startup with
    /// "Short option names must be unique for each argument".
    #[arg(long, default_value = "error")]
    needle: String,

    /// Show matching line numbers
    #[arg(short = 'n', long)]
    line_numbers: bool,

    /// Maximum matches to print
    #[arg(short, long, default_value_t = 100)]
    limit: usize,
}

fn main() -> ExitCode {
    let args = Args::parse();

    match run(&args) {
        Ok(n) => {
            println!("{n} matches");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}

fn run(args: &Args) -> Result<usize, String> {
    match args.file.as_deref() {
        // No file given: make one, so `cargo run -p ch19-cli` does something.
        None => {
            let dir = std::env::temp_dir().join("seeing-rust-ch19");
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let p = dir.join("sample.log");
            std::fs::write(&p, SAMPLE).map_err(|e| e.to_string())?;
            println!("(no file given; using {})", p.display());
            let text = SAMPLE.to_string();
            Ok(filter(&text, args))
        }
        Some(path) if path.as_os_str() == "-" => {
            // Read stdin when no file was given -- this is what makes a tool
            // composable:  cat app.log | logfilter - --needle 500
            let stdin = io::stdin();
            let mut buf = String::new();
            for line in stdin.lock().lines() {
                let line = line.map_err(|e| e.to_string())?;
                buf.push_str(&line);
                buf.push('\n');
            }
            Ok(filter(&buf, args))
        }
        Some(path) => {
            let text = std::fs::read_to_string(path)
                .map_err(|e| format!("{}: {e}", path.display()))?;
            Ok(filter(&text, args))
        }
    }
}

fn filter(text: &str, args: &Args) -> usize {
    let mut shown = 0;
    for (i, line) in text.lines().enumerate() {
        if shown >= args.limit {
            break;
        }
        if line.contains(&args.needle) {
            if args.line_numbers {
                println!("{}: {line}", i + 1);
            } else {
                println!("{line}");
            }
            shown += 1;
        }
    }
    shown
}

const SAMPLE: &str = "\
2026-01-14T09:12:30Z level=info  service=api  msg=\"started\"
2026-01-14T09:12:33Z level=info  service=api  msg=\"listening on :8080\"
2026-01-14T09:12:41Z level=warn  service=api  msg=\"retry budget low\"
2026-01-14T09:12:42Z level=error service=api  msg=\"upstream failed\" code=503
2026-01-14T09:12:43Z level=info  service=api  msg=\"retry 1\"
2026-01-14T09:12:45Z level=error service=api  msg=\"upstream failed\" code=503
2026-01-14T09:12:46Z level=info  service=api  msg=\"recovered\"
";
