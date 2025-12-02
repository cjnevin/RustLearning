use std::{collections::HashMap, io};

fn main() {
    scores();
    count_words();
    add_employees();
}

fn scores() -> HashMap<String, i32> {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // get value or return zero
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name} score is: {score}");

    // override
    scores.insert(String::from("Blue"), 25);

    // insert only if not found
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    return scores;
}

fn count_words() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

fn add_employees() {
    let mut department: HashMap<String, Vec<String>> = HashMap::new();

    println!("Type 'add [name] to [department]' to start, type 'exit' to finish");

    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let line = line.trim();
        if line.eq_ignore_ascii_case("exit") {
            println!("Exiting employee entry.");
            println!("Employees: {department:?}");
            break;
        }

        if line.is_empty() {
            println!("Empty input. Please try again.");
            continue;
        }

        let mut words = line.split_whitespace();

        let add = words.next();
        if !add.map_or(false, |w| w.eq_ignore_ascii_case("add")) {
            println!("Error: Invalid command. Expected 'Add'.");
            continue;
        };

        let name = match words.next() {
            Some(n) => n.to_string(),
            None => {
                println!("Error: Missing employee name.");
                continue;
            }
        };

        let to_word = words.next();
        if !to_word.map_or(false, |w| w.eq_ignore_ascii_case("to")) {
            println!("Error: Invalid command format. Expected 'to' after name.");
            continue;
        }

        let department_name = words.collect::<Vec<&str>>().join(" ");

        department
            .entry(department_name)
            .or_insert_with(Vec::new)
            .push(name);
    }
}
