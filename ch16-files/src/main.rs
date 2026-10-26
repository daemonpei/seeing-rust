//! Chapter 16: File I/O and Paths
//!
//! Everything happens in a scratch directory under the system temp dir, so the
//! program is safe to run repeatedly and never touches your project.

use std::fs;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    let root = scratch_dir()?;
    println!("working in {}\n", root.display());

    read_whole_and_by_line(&root)?;
    write_append_and_replace(&root)?;
    paths(&root);
    error_kinds(&root)?;
    println!("\ntotal bytes under the dir: {}", total_size(&root)?);
    Ok(())
}

fn scratch_dir() -> std::io::Result<PathBuf> {
    let dir = std::env::temp_dir().join(format!("seeing-rust-ch16-{}", std::process::id()));
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn read_whole_and_by_line(root: &Path) -> std::io::Result<()> {
    // Whole file, when it is small and text.
    let log = root.join("app.log");
    fs::write(
        &log,
        "level=info service=api\nlevel=error service=api code=500\nlevel=error service=web\n",
    )?;

    let text = fs::read_to_string(&log)?;
    println!("read_to_string: {} bytes", text.len());

    // Line by line, when it may be large.
    let file = fs::File::open(&log)?;
    let reader = BufReader::new(file);
    for (n, line) in reader.lines().enumerate() {
        let line = line?; // each line is its own Result
        if line.contains("level=error") {
            println!("  line {}: {line}", n + 1);
        }
    }
    Ok(())
}

fn write_append_and_replace(root: &Path) -> std::io::Result<()> {
    let report = root.join("report.txt");

    // Truncate and write (creates the file if missing).
    fs::write(&report, "service=api\n")?;

    // Append: needs OpenOptions, because fs::write always truncates.
    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&report)?;
    writeln!(f, "port=8080")?;

    // Atomic-ish replace: write a temp file, then rename over the target.
    let tmp = report.with_extension("tmp");
    fs::write(&tmp, "service=api\nport=8080\nrestarted=true\n")?;
    fs::rename(&tmp, &report)?; // rename is atomic on the same filesystem

    println!("\nafter replace:\n{}", fs::read_to_string(&report)?);
    Ok(())
}

fn paths(root: &Path) {
    let base = root.join("configs");
    let full: PathBuf = base.join("service.json"); // correct separator per OS

    println!("full          {full:?}");
    println!("file_name     {:?}", full.file_name());
    println!("extension     {:?}", full.extension());
    println!("parent        {:?}", full.parent());
    println!("with_extension {:?}", full.with_extension("bak"));

    // Accept &Path in signatures: it takes &str, String and PathBuf alike.
    println!("load(&Path)   {:?}", load(&full));
    println!("load(&str)    {:?}", load(Path::new("a.json")));
}

fn load(p: &Path) -> std::io::Result<String> {
    fs::read_to_string(p)
}

fn error_kinds(root: &Path) -> std::io::Result<()> {
    let missing = root.join("does-not-exist.json");

    // Prefer matching the error over a pre-check: TOCTOU-free.
    match fs::read_to_string(&missing) {
        Ok(text) => println!("\nread {} bytes", text.len()),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            println!("\nno config file; using defaults");
        }
        Err(e) => return Err(e),
    }

    fs::create_dir_all(root.join("reports/2026"))?; // like mkdir -p
    let meta = fs::metadata(root.join("report.txt"))?;
    println!("{} bytes", meta.len());
    println!("is file: {}", meta.is_file());
    Ok(())
}

fn total_size(dir: &Path) -> std::io::Result<u64> {
    let mut total = 0;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        if meta.is_file() {
            total += meta.len();
        } else if meta.is_dir() {
            total += total_size(&entry.path())?;
        }
    }
    Ok(total)
}
