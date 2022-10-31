// Write a program that outputs all possibilities to put + or - or nothing between the numbers 1,2,…,9 (in this order) 
// such that the result is 100. For example 1 + 2 + 3 - 4 + 5 + 6 + 78 + 9 = 100.
//use eval
use eval::{eval};


fn main(){
    dfs(1, "", "");
}

// define a depth first search that takes a number and an operator or empty string and checks after 9 numbers if the sum is 100
// if the sum is 100, print the string
fn dfs(n: i32, op: &str, s: &str) {
    if n == 9 {
        if eval(s).unwrap() == 100 {
            println!("{:}", s);
        }
        return;
    }
    dfs(n + 1, "+", &format!("{:}{:}{:}", s, op, n + 1));
    dfs(n + 1, "-", &format!("{:}{:}{:}", s, op, n + 1));
    dfs(n + 1, "", &format!("{:}{:}{:}", s, op, n + 1));
}