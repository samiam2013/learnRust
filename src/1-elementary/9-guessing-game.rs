/*
    9. Write a guessing game where the user has to guess a secret number.
    After every guess the program tells the user whether their number was
    too large or too small. At the end the number of tries needed should
    be printed. It counts only as one try if they input the same number
    multiple times consecutively.
*/
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::collections::HashSet;
use std::io::stdin;
use std::io::{self, Write};
use std::time::SystemTime;

fn main() {
    println!("Welcome to the guessing game. Pick a number 1-100");
    let epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut rng = Pcg32::seed_from_u64(epoch);
    let random_number: u32 = rng.gen_range(1..100);
    let mut guesses = HashSet::new();
    loop {
        let mut guess = String::new();
        print!("Your number: ");
        io::stdout().flush().unwrap();
        stdin().read_line(&mut guess).unwrap();
        guess.truncate(guess.trim().len());
        let guess_n = guess.parse::<u32>().unwrap();
        println!("your guess: {:}", guess_n);
        guesses.insert(guess_n);
        if guess_n == random_number {
            break;
        } else if guess_n > random_number {
            println!("number too high. try again.");
        } else {
            println!("number was too low. try again");
        }
    }
    println!("Correct! number of guesses: {:}", guesses.len());
}
