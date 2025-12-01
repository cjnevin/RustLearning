const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    mutability();
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
    shadowing();
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
