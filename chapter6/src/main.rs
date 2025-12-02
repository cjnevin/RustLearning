#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("{self:?}");
    }
}

fn main() {
    ip_address();
    message_call();
    match_control_flow();
    matching_with_option();
}

fn ip_address() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{home:?}");
    println!("{loopback:?}");
}

fn message_call() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let q = Message::Quit;
    q.call();

    let mv = Message::Move { x: 30, y: 50 };
    mv.call();

    let c = Message::ChangeColor(255, 40, 40);
    c.call();
}

fn match_control_flow() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("None");
            None
        }
        Some(i) => {
            println!("Plus one {}", i + 1);
            Some(i + 1)
        }
    }
}

fn matching_with_option() {
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}
