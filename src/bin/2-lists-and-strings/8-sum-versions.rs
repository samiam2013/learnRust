/* 8. Write three functions that compute the sum of the numbers in a list: using 
    a for-loop, 
    a while-loop and 
    recursion. 
    (Subject to availability of these constructs in your language of choice.) */

fn main() {
    let mut list = vec!(1, 69, 42, 73);
    println!("for_sum: {:}", for_sum(&list));
    println!("while_sum: {:}", while_sum(&list));
    println!("recursive_sum: {:}", recursive_sum(&mut list,0));
}

fn for_sum(list: &Vec<i32>) -> i32 {
    let mut sum = 0; 
    for n in list {
        sum += n;
    }
    return sum;
}

fn while_sum(list: &Vec<i32>) -> i32 {
    let mut sum = 0; 
    let mut i = 0;
    while i < list.len() {
        sum += list[i];
        i += 1;
    }
    return sum;
}

fn recursive_sum(list: &mut Vec<i32>, sum: i32) -> i32 {
    if list.len() == 1 {
        return list[0] + sum;
    }
    let elem = list.pop().unwrap(); 
    return recursive_sum(list, sum + elem)
}