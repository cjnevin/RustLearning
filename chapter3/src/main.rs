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
    if_expression();
    elseif_expression();
    one_line_if_else();
    loop_until_result();
    counting_up_loop();
    liftoff_while_loop();
    while_for_loop();
    for_in_loop();
    for_in_loop_shorthand();
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

fn if_expression() {
    let number = 3;
    if number < 5 {
        println!("if condition was true");
    } else {
        println!("if condition was false");
    }
}

fn elseif_expression() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn one_line_if_else() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

fn loop_until_result() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result)
}

fn counting_up_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn liftoff_while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn while_for_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_in_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn for_in_loop_shorthand() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
