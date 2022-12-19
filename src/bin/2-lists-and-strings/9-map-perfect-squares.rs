/* 9. Write a function on_all that applies a function to every element of a list.
Use it to print the first twenty perfect squares.
The perfect squares can be found by multiplying each natural number with itself.
The first few perfect squares are 1*1= 1, 2*2=4, 3*3=9, 4*4=16.
Twelve for example is not a perfect square because there is no natural number m so that m*m=12.
(This question is tricky if your programming language makes it difficult to pass functions as arguments.) */

fn main() {
    let mut list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    on_all(&mut list, square);
    println!("squares: {:?}", list);
}

fn on_all(list: &mut Vec<i32>, f: fn(i32) -> i32) {
    let mut i = 0;
    while i < list.len() {
        let copy = list[i].to_owned();
        list[i] = f(copy);
        i += 1;
    }
}

fn square(n: i32) -> i32 {
    return n * n;
}
