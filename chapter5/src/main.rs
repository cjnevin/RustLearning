struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main() {
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

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    let _subject = AlwaysEqual;
}
