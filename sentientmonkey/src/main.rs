use std::io;

fn main() {
    println!("Tell me your name.");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .ok()
        .expect("Failed to read line");

    name.pop();

    println!("Hello, {}!", name);
}
