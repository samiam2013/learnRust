// 2. Write a program that asks the user for their name and greets them with their name.

use std::io::stdin;

fn main() {
    // variable for user input 
    let mut input = String::new();
    println!("What is your name?: ");
    let _ = stdin().read_line(&mut input).unwrap();
    println!("Hello, {}", input);
}