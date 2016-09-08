extern crate list;

use list::{write, eval, read};

pub fn main() {
    println!("Welcome to List. Use Ctrl-C to exit.");

    loop {
        write(eval(read()));
    }
}
