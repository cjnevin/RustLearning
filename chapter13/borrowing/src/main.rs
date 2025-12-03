use std::thread;

fn main() {
    let mut list = vec![1, 2, 3];

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling mutable closure: {list:?}");

    // Using move to force the closure for the thread to take ownership of list
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // list no longer usable at this point
}
