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

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("{self:?}");
    }
}

fn main() {
    ip_address();
    message_call();
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
