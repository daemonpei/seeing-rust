//! Chapter 11: Structs, Enums and Domain Models

#[derive(Debug, Clone, PartialEq)]
struct Server {
    name: String,
    port: u16,
    healthy: bool,
}

#[derive(Debug)]
enum Job {
    Queued,
    Running { worker: u32, progress: f32 },
    Done(u64),      // milliseconds taken
    Failed(String), // the error message
}

fn main() {
    // ---- construction and Debug ------------------------------------------
    let s = Server {
        name: String::from("api-1"),
        port: 8080,
        healthy: true,
    };
    println!("{} -> {}", s.name, s.port);
    println!("{s:#?}"); // Debug pretty-print

    // ---- struct update syntax moves the unlisted fields -------------------
    let base = Server {
        name: String::from("api"),
        port: 8080,
        healthy: true,
    };
    let next = Server {
        port: 9090,
        ..base
    };
    println!("{} -> {}", next.name, next.port);
    // `base` is partially moved: its `name` String went to `next`
    // println!("{:?}", base);   // error: use of partially moved value

    // ---- enums carry data, and match destructures it ----------------------
    let jobs = [
        Job::Queued,
        Job::Running {
            worker: 3,
            progress: 0.42,
        },
        Job::Done(1_240),
        Job::Failed(String::from("timeout")),
    ];
    for job in &jobs {
        println!("{:24} {}", format!("{job:?}"), describe(job));
    }

    // `..` in a pattern ignores the fields you do not need, and keeps a match
    // compiling when a variant gains a field.
    if let Some(Job::Running { progress, .. }) = jobs.get(1) {
        println!("- one field at a time: {:.2}", progress);
    }

    // ---- PartialEq lets you compare models directly ----------------------
    println!(
        "same server? {}",
        s == Server {
            name: String::from("api-1"),
            port: 8080,
            healthy: true
        }
    );
}

fn describe(job: &Job) -> String {
    match job {
        Job::Queued => "waiting".to_string(),
        Job::Running { worker, progress } => {
            format!("{:.0}% done on worker {worker}", progress * 100.0)
        }
        Job::Done(ms) => format!("finished in {ms} ms"),
        Job::Failed(msg) => format!("failed: {msg}"),
    }
}

/// The chapter closes by showing that `Option` and `Result` are just enums. They
/// cannot be redeclared in the crate root, so they live in their own module --
/// which has the side effect of proving the point: nothing special is happening.
#[allow(dead_code)]
mod stdlib_shape {
    pub enum Option<T> {
        None,
        Some(T),
    }

    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
