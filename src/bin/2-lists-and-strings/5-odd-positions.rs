/* 5. Write a function that returns the elements on odd positions in a list. */

fn main() {
    // starting from 0th index, by index oddity
    let list = vec!["even0", "odd1", "even2", "odd3"];
    println!("odd items: {:?}", odd_pos(list))
}

fn odd_pos<T: std::marker::Copy>(list: Vec<T>) -> Vec<T> {
    let mut odds = vec![];
    let mut i = 1;
    while i < list.len() {
        let odd_val = list[i];
        odds.push(odd_val);
        i += 2;
    }
    return odds;
}
