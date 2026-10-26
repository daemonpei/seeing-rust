//! Chapter 23: Threads and Concurrent Work

use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    // ---- a thread is a closure ------------------------------------------
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("worker: {i}");
            thread::sleep(Duration::from_millis(10));
        }
    });
    handle.join().unwrap(); // main waits here
    println!("main: done\n");

    // ---- move: the thread owns what it touches --------------------------
    let data = vec![1, 2, 3];
    // Without `move`, the closure would borrow a local that may not outlive
    // the thread -- and the compiler rejects that with a lifetime error, not a
    // race:
    //
    //   error[E0597]: `data` does not live long enough
    //   note: closure may outlive the current function, but it borrows `data`
    //
    let handle = thread::spawn(move || {
        println!("{:?}", data); // data now belongs to the thread
    });
    handle.join().unwrap();

    // ---- scoped threads borrow instead of moving ------------------------
    let numbers = vec![1, 2, 3, 4];
    let threshold = 2;

    let (big, small): (Vec<i32>, Vec<i32>) = thread::scope(|s| {
        let a = s.spawn(|| {
            numbers
                .iter()
                .copied()
                .filter(|n| *n > threshold)
                .collect()
        });
        let b = s.spawn(|| {
            numbers
                .iter()
                .copied()
                .filter(|n| *n <= threshold)
                .collect()
        });
        (a.join().unwrap(), b.join().unwrap())
    });

    println!("\nscoped: {big:?} {small:?}"); // [3, 4] [1, 2]
    println!("numbers still usable: {numbers:?}");

    // ---- one thread per file --------------------------------------------
    let files = make_files();
    let handles: Vec<_> = files
        .iter()
        .map(|path| {
            let path = path.clone();
            thread::spawn(move || (path.clone(), fs::read_to_string(&path)))
        })
        .collect();

    println!();
    for h in handles {
        let (path, result) = h.join().unwrap();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        match result {
            Ok(text) => println!("{name}: {} bytes", text.len()),
            Err(e) => println!("{name}: {e}"),
        }
    }

    // ---- do not hardcode the core count ---------------------------------
    let workers = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    println!("\navailable_parallelism() = {workers}");
    println!("(a container with a CPU quota reports the quota, not the host)");
}

fn make_files() -> Vec<PathBuf> {
    let dir = std::env::temp_dir().join("seeing-rust-ch23");
    fs::create_dir_all(&dir).unwrap();
    let mut out = Vec::new();
    for (i, body) in ["alpha", "beta beta", "gamma gamma gamma"].iter().enumerate() {
        let p = dir.join(format!("f{i}.txt"));
        fs::write(&p, body).unwrap();
        out.push(p);
    }
    out
}
