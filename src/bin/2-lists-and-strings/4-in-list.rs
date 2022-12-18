/* 4. Write a function that checks whether an element occurs in a list. */

fn main() {
    let list = vec!("one", "two", "red","blue", "green");
    if in_list(list, "green") {
        println!("green is in the list");
        return
    }
    println!("green is not in the list")
}

fn in_list<T: std::cmp::PartialEq>(list: Vec<T>, item:T) -> bool {
    for candidate in list {
        if item == candidate {
            return true
        }
    }
    return false
}