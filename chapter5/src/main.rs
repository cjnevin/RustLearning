struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    user_mutation();

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    let _subject = AlwaysEqual;

    area_calculations();
}

fn user_mutation() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    println!(
        "{} {} {} {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    // Remaining fields automatically added
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!(
        "{} {} {} {}",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );
}

fn area_calculations() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );

    println!("Rect is {rect2:?}");

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect3);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
