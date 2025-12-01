fn main() {
    string_mutation();
    ahoy_world();
    clone_example();
    integer_does_not_need_clone();
    ownership_take_examples();
    ownership_give_example();
    returning_ownership_of_parameters();
    borrowing();
    slicing();
}

fn string_mutation() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // this will print `hello, world!`
}

fn ahoy_world() {
    let mut s = String::from("hello");
    println!("{s}, world!");

    s = String::from("ahoy");

    println!("{s}, world!");
}

fn clone_example() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

fn integer_does_not_need_clone() {
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}

fn ownership_take_examples() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // Because i32 implements the Copy trait,
    // x does NOT move into the function,
    // so it's okay to use x afterward.
} // Here, x goes out of scope, then s. However, because s's value was moved,
// nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn ownership_give_example() {
    let s1 = gives_ownership(); // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn returning_ownership_of_parameters() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn borrowing() {
    let s1 = String::from("hello");
    let len = borrowing_calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let mut s2 = String::from("hello");
    change(&mut s2);

    let s3 = String::from("hello");

    let r1 = &s3;
    let r2 = &s3;

    println!("{r1}, {r2}");

    let mut s4 = String::from("hello");

    {
        let r4 = &mut s4;
        println!("{r4}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r5 = &mut s4;
    println!("{r5}");
}

fn borrowing_calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn slicing() {
    length_slicing();
    exact_slicing();
    improved_slicing();
}

fn length_slicing() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("{}", &s[(word + 1)..]);

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn exact_slicing() {
    let s2 = String::from("hello world");
    let hello = &s2[0..5];
    let world = &s2[6..11];
    println!("{hello} {world}");
}

fn improved_slicing() {
    let my_string = String::from("hello world");

    // `first_word_improved` works on slices of `String`s, whether partial or whole.
    let word = first_word_improved(&my_string[0..6]);
    let word = first_word_improved(&my_string[..]);
    // `first_word_improved` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word_improved(&my_string);

    let my_string_literal = "hello world";

    // `first_word_improved` works on slices of string literals, whether partial or
    // whole.
    let word = first_word_improved(&my_string_literal[0..6]);
    let word = first_word_improved(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_improved(my_string_literal);
}

fn first_word_improved(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

#[cfg(test)]
mod tests {
    #[test]
    fn slicing() {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
    }
}
