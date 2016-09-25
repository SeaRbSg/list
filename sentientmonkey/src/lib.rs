#![feature(io)]
use std::io;
use std::io::prelude::*;

#[derive(Debug,PartialEq,Clone)]
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

pub fn read() -> Result<Lval, String> {
    print!("> ");
    io::stdout().flush().unwrap();


    let mut input = "".to_string();
    for char in io::stdin().chars() {
        let c = char.unwrap();
        if c == '\n' {
            break;
        }
        input.push(c);
    }

    let p = input.parse::<i64>();
    match p {
        Ok(i) => Ok(Lval::Num(i)),
        Err(_) => Err(format!("Could not parse '{}'", input).to_string()),
    }
}

pub fn eval(l: Result<Lval, String>) -> Result<Lval, String> {
    l
}

pub fn write(l: Result<Lval, String>) {
    match l {
        Ok(v) => println!("{:?}", v.to_string()),
        Err(v) => println!("Error: {}", v),
    }
}
