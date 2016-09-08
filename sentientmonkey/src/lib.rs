use std::io::{self, Write};

pub fn read() -> String {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    input.pop();

    return input;
}

pub fn eval(s: String) -> String {
    s
}

pub fn write(s: String) {
    println!("{}", s);
}
