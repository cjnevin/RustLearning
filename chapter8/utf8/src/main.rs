fn main() {
    println!("{:?}", new());
    to_string();
    hello();
    println!("{}", appending());
    println!("{}", appending_slice_no_drop());
    println!("{}", appending_char());
    println!("{}", move_using_plus());
    tic_tac_toe();
    tic_tac_toe_format();
    iterate_utf8_string_chars();
    iterate_utf8_string_bytes();
}

fn new() -> String {
    let s = String::new();
    return s;
}

fn to_string() {
    let data = "initial contents";

    let _s1 = data.to_string();

    // The method also works on a literal directly:
    let _s2 = "initial contents".to_string();

    // Equivalent to
    let _s3 = String::from("initial contents");
}

fn hello() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn appending() -> String {
    let mut s = String::from("foo");
    s.push_str("bar");
    return s;
}

fn appending_slice_no_drop() -> String {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    return s1;
}

fn appending_char() -> String {
    let mut s = String::from("lo");
    s.push('l');
    return s;
}

fn move_using_plus() -> String {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    return s3;
}

fn tic_tac_toe() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");
}

fn tic_tac_toe_format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // uses references
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}

fn iterate_utf8_string_chars() {
    for c in "Зд".chars() {
        println!("{c}");
    }
}

fn iterate_utf8_string_bytes() {
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
