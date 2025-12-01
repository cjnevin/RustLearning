const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

mod invalid_index;

fn main() {
    mutability();
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
    shadowing();
    tuples();
    type_inferred_array();
    typed_array();
    repeated_item_array();
    expression();
    invalid_index::invalid_array();
}

fn mutability() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{} {} {}", tup.0, tup.1, tup.2);
}

fn type_inferred_array() {
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[4]);
}

fn typed_array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[4]);
}

fn repeated_item_array() {
    let a = [3; 5];
    println!("{}", a[4]);
}

fn expression() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}
