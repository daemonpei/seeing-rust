//! Chapter 6: Loops and Repeated Work

// The index loop is the *subject* of this chapter, not an accident, so the lint
// that wants `enumerate` everywhere is switched off here on purpose.
#![allow(clippy::needless_range_loop)]

fn main() {
    let hosts = ["api-1", "api-2", "api-3"];

    for host in hosts {
        // borrows the array
        println!("checking {host}");
    }

    for i in 0..hosts.len() {
        // half-open: 0,1,2 (never 3)
        println!("{i}: {}", hosts[i]);
    }

    for (i, host) in hosts.iter().enumerate() {
        // index + value together
        println!("{i} -> {host}");
    }

    println!();

    // `loop` as an expression: break carries the value out
    println!("first odd port: {:?}", first_available(&[2, 4, 7, 8]));
    println!("none available : {:?}", first_available(&[2, 4, 6]));

    println!();

    // labelled break leaves both loops
    grid_search();

    println!();

    // a loop and a pipeline that produce the same Vec
    let ports = vec![80, 443, 8080, 9090];

    let mut big = Vec::new();
    for p in &ports {
        if *p > 1000 {
            big.push(*p);
        }
    }
    let big_pipeline: Vec<&u16> = ports.iter().filter(|p| **p > 1000).collect();

    println!("loop     : {big:?}");
    println!("pipeline : {big_pipeline:?}");
}

/// `break Some(port)` — the loop itself produces the value.
fn first_available(ports: &[u16]) -> Option<u16> {
    let mut i = 0;
    loop {
        let port = *ports.get(i)?; // None ends the function
        if port % 2 == 1 {
            break Some(port); // the loop evaluates to this
        }
        i += 1;
    }
}

fn grid_search() {
    // An 8x8 board where each cell holds its own index, so "found at r,c"
    // is checkable by eye.
    let mut board = [[0i32; 8]; 8];
    for row in 0..8 {
        for col in 0..8 {
            board[row][col] = (row * 8 + col) as i32;
        }
    }
    let target = 21;
    let blocked = 9;

    'search: for row in 0..8 {
        for col in 0..8 {
            if board[row][col] == target {
                println!("found at {row},{col}");
                break 'search; // leaves BOTH loops
            }
            if board[row][col] == blocked {
                continue 'search; // skip to the next row
            }
        }
    }
}
