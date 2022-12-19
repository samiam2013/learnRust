/* 10. Write a function that concatenates two lists. [a,b,c], [1,2,3] → [a,b,c,1,2,3] */

fn main() {
    let list1 = vec!('a', 'b', 'c'); 
    let list2 = vec!('1', '2', '3');
    println!("concatenated: {:?}", concat(list1, list2))
}

fn concat(one: Vec<char>, two: Vec<char>) -> Vec<char> {
    let mut new: Vec<char> = vec!();
    let mut clone = one.clone();
    new.append(&mut clone);
    clone = two.clone();
    new.append(&mut clone);
    new
}