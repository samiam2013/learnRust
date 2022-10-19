// 3. Modify the previous program such that only the users Alice and Bob are greeted with their names.
use std::io::stdin;

fn main() {
    // variable for user input 
    let mut input = String::new();
    println!("What is your name?: ");
    let _ = stdin().read_line(&mut input).unwrap();
    if input == "Alice" || input == "Bob" {
        println!("Hello, {}", input);
    }
}