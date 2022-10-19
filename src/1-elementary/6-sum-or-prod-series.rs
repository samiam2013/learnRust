use std::io::stdin;
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("do you want a [P]roduct or [S]um?: ");
    io::stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();                         
    input.truncate(input.trim().len());
    println!("some input: {:?}", input.);
}