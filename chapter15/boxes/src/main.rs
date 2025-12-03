use crate::List::{Cons, Nil};
use std::ops::Deref;

// enum List {
//     Cons(i32, List), // Original, won't compile -- recursive loop on List
//     Nil,
// }

#[derive(Debug)]
enum List {
    /// The Cons variant needs the size of an i32 plus the space to store
    /// the box’s pointer data. The Nil variant stores no values, so it needs
    /// less space on the stack than the Cons variant.
    ///
    /// Boxes provide only the indirection and heap allocation;
    /// they don’t have any other special capabilities,
    /// like those we’ll see with the other smart pointer types.
    /// They also don’t have the performance overhead that these special capabilities incur,
    /// so they can be useful in cases like the cons list where the indirection is the only
    /// feature we need.
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    // let list = Cons(1, Cons(2, Cons(3, Nil))); // Won't work
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Boxed list {list:?}");

    follow_reference_of_value();
    follow_reference_of_value_in_box();
    follow_reference_of_value_in_my_box();

    hello_my_box();
}

fn follow_reference_of_value() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn follow_reference_of_value_in_box() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implement deref so we can reference using *y below

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn follow_reference_of_value_in_my_box() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // ... behind the scenes Rust actually ran this code:
    // *(y.deref())
    assert_eq!(5, *y);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn hello_my_box() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // If Rust didn’t implement deref coercion, we would have to
    // write the code in Listing 15-13 instead of the code in
    // Listing 15-12 to call hello with a value of type &MyBox<String>.
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
