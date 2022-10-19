// Write a program that asks the user for a number n and prints
//  the sum of the numbers 1 to n
use std::io::stdin;
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.truncate(input.trim().len());
    println!("some input: {:?}", input);
    let conv_num = input.parse::<i64>();
    let num_int = conv_num.unwrap();
    println!("converted number: {:?}", num_int);
    let mut sum: i64 = 0;
    for i in 1..num_int {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    println!("fizzbuzz triangle sum: {:?}", sum);
}
