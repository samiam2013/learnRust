/* 6. Write a function that computes the running total of a list. */

fn main() {
    let list = vec![103, 21, 1, 5];
    println!("running totals: {:?}", running_totals(list));
}

fn running_totals(list: Vec<i32>) -> Vec<i32> {
    let mut r_totals: Vec<i32> = vec![];
    let mut sum: i32 = 0;
    for item in list {
        sum += item;
        r_totals.push(sum);
    }
    return r_totals;
}
