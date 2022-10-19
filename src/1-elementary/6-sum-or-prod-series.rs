use std::io::stdin;
use std::io::{self, Write};

fn main() {
    let mut prod_or_sum = String::new();
    print!("do you want a [P]roduct or [S]um?: ");
    io::stdout().flush().unwrap();
    stdin().read_line(&mut prod_or_sum).unwrap();                         
    prod_or_sum.truncate(prod_or_sum.trim().len());
    let select_char =prod_or_sum.to_lowercase().chars().nth(0).unwrap();
    if select_char != 'p' && select_char != 's' {
        println!("some prod_or_sum: {:?}", select_char );
        panic!("product or sum selection unparsable");
    }
    // ask for the nth element 1..n
    let mut n_strval = String::new();
    print!("operate on elements 1 to n where n is: ");
    io::stdout().flush().unwrap();
    stdin().read_line(&mut n_strval).unwrap();
    n_strval.truncate(n_strval.trim().len());
    let n_val = n_strval.parse::<i64>().unwrap();
    // TODO replace with pattern match?
    match select_char {
        'p' => println!("product: {:?}", product(n_val)),
        's' => println!("product: {:?}", triangle(n_val)),
        _ => panic!("case unhandled!"),
    }
}

fn product(n: i64) -> i64 {
    let mut prod = 1;
    for i in 1..n {
        prod *= i;
    }
    prod
}

fn triangle(n: i64) -> i64 {
    let sum = (n*(n+1))/2;
    sum
}