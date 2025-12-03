use crate::List::{Cons, Nil};

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
}
