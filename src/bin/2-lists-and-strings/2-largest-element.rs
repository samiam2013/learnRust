/* 2. Write a function that returns the largest element in a list. */

fn main() {
    let list: Vec<String> = ["item1".to_string(), "longest item".to_string()].to_vec();
    let longest = find_longest(list);
    println!("longest item: {:}", longest)
}

fn find_longest(list: Vec<String>) -> String {
    let mut longest: String = String::new();
    for li in list {
        if li.len() > longest.len() {
            longest = li
        }
    }
    longest
}
