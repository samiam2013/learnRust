/* 11. Write a function that combines two lists by alternatingly taking elements, e.g. [a,b,c], [1,2,3] → [a,1,b,2,c,3]. */

use core::{panic};

fn main() {
    let list1 = vec!['a', 'b', 'c'];
    let list2 = vec!['1', '2', '3'];
    println!("coallated: {:?}", coallate(list1, list2))
}

fn coallate(one: Vec<char>, two: Vec<char>) -> Result<Vec<char>, String> {
    if one.len() != two.len() {
        panic!("vectors must be the same length");
    }
    let mut coallated: Vec<char> = Vec::new();
    for i in 0..one.len() {
        coallated.push(one[i]);
        coallated.push(two[i]);       
    }
    return Ok(coallated);
}