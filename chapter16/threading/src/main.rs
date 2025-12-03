use std::{thread, time::Duration};

fn main() {
    // spawn_threads_and_execute_as_many_as_possible();
    spawn_threads_and_wait_til_complete_concurrent();
    spawn_threads_and_wait_til_complete_sequential();
    spawn_capturing_variable();
}

// Have to move `v` to spawned thread or it will be dropped before thread completes
fn spawn_capturing_variable() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}

// Background thread will finish executing early once main thread completes
fn spawn_threads_and_execute_as_many_as_possible() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

// Add join to wait until all background tasks finish
fn spawn_threads_and_wait_til_complete_concurrent() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("[concurrent] hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("[concurrent] hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

// Add join to wait until all background tasks finish before starting main thread work
fn spawn_threads_and_wait_til_complete_sequential() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("[sequential] hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("[sequential] hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
