//! Chapter 24: Channels and Message Passing

use std::sync::mpsc::{self, channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum Job {
    Task(String),
    Quit,
}

fn main() {
    simple();
    bounded();
    worker_pool();
    fan_in();
}

/// The basic shape: one producer, one consumer, the channel closing itself.
fn simple() {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        for job in ["alpha", "beta", "gamma"] {
            tx.send(job.to_string()).unwrap();
        }
        // tx is dropped here, which closes the channel
    });

    println!("-- simple");
    for received in rx {
        // iterates until the channel closes
        println!("got {received}");
    }
}

/// `sync_channel(bound)` blocks the sender once the buffer is full.
fn bounded() {
    println!("\n-- bounded");
    let (tx, rx) = mpsc::sync_channel::<Job>(2); // buffer of 2

    let producer = thread::spawn(move || {
        for i in 0..6 {
            let start = std::time::Instant::now();
            tx.send(Job::Task(format!("job-{i}"))).unwrap();
            // With a bound of 2, this send waits once the buffer is full.
            println!("  sent job-{i} (blocked {:?})", start.elapsed());
        }
        tx.send(Job::Quit).unwrap();
    });

    while let Ok(job) = rx.recv() {
        match job {
            Job::Quit => break,
            Job::Task(name) => {
                thread::sleep(Duration::from_millis(15)); // a slow consumer
                println!("  handled {name}");
            }
        }
    }
    producer.join().unwrap();

    // This is the feature people forget they wanted: if the consumer is slow,
    // the producer slows down instead of queueing jobs in memory until the
    // process is killed by the OOM killer.
}

/// Several consumers sharing one receiver: wrap it in Arc<Mutex<_>>.
fn worker_pool() {
    println!("\n-- worker pool");
    let (tx, rx) = channel::<Job>();
    let rx = Arc::new(Mutex::new(rx));

    let mut workers = Vec::new();
    for id in 0..4 {
        let rx = Arc::clone(&rx);
        workers.push(thread::spawn(move || loop {
            // Lock only to take one job, then release.
            let job = rx.lock().unwrap().recv();
            match job {
                Ok(Job::Task(name)) => println!("  worker {id}: {name}"),
                Ok(Job::Quit) | Err(_) => break,
            }
        }));
    }

    for i in 0..8 {
        tx.send(Job::Task(format!("job-{i}"))).unwrap();
    }
    for _ in 0..4 {
        tx.send(Job::Quit).unwrap();
    }
    for w in workers {
        w.join().unwrap();
    }
}

/// Many producers, one consumer.
fn fan_in() {
    println!("\n-- fan in");
    let (tx, rx) = mpsc::channel::<(String, u16)>();

    let urls = ["api-1", "api-2", "api-3"];
    for url in urls {
        let tx: Sender<(String, u16)> = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(5));
            tx.send((url.to_string(), 200)).unwrap();
        });
    }
    drop(tx); // drop the original, or the loop never ends

    let mut seen: Vec<String> = Vec::new();
    for (url, status) in rx {
        println!("  {url}: {status}");
        seen.push(url);
    }
    seen.sort();
    println!("  collected {} results", seen.len());
}
