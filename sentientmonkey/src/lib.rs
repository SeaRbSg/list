use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum Lval {
    Num(i64),
}

impl Lval {
    fn to_string(&self) -> String {
        match *self {
            Lval::Num(i) => i.to_string(),
        }
    }
}

pub fn read() -> Result<Lval, io::Error> {
    print!("> ");
    io::stdout().flush().unwrap();


    let input: Result<i64, io::Error> = std::io::stdin()
        .bytes()
        .map(|byte| byte as i64);

    return input.and_then(|i| Lval::Num(i));
}

pub fn eval(l: Result<Lval, io::Error>) -> Result<Lval, io::Error> {
    l
}

pub fn write(l: Result<Lval, io::Error>) {
    match l {
        Ok(v) => println!("{:?}", v.to_string()),
        Err(v) => println!("Error: {}", v),
    }
}
