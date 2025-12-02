fn main() {
    println!("{:?}", new());
    println!("{:?}", shorthand());
    println!("{:?}", pushed());
    reading_elements();
    iteration();
    iteration_mutation();
    enumeration();
}

fn new() -> Vec<i32> {
    let v: Vec<i32> = Vec::new();
    return v;
}

fn shorthand() -> Vec<i32> {
    let v = vec![1, 2, 3];
    return v;
}

fn pushed() -> Vec<i32> {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    return v;
}

fn reading_elements() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let sixth: Option<&i32> = v.get(5);
    match sixth {
        Some(sixth) => println!("The sixth element is {sixth}"),
        None => println!("There is no sixth element."),
    }
}

fn iteration() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
}

fn iteration_mutation() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}

fn enumeration() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
