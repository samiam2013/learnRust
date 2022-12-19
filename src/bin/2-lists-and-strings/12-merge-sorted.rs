/*12. Write a function that merges two sorted lists into a new sorted list. [1,4,6],[2,3,5] → [1,2,3,4,5,6].
You can do this quicker than concatenating them followed by a sort. */

fn main() {
    let list1 = vec![1, 4, 6];
    let list2 = vec![2, 3, 5];
    println!("merged: {:?}", merge_sorted(list1, list2));
}

fn merge_sorted(one: Vec<i32>, two: Vec<i32>) -> Vec<i32> {
    let mut merged: Vec<i32> = Vec::new();
    let mut one_i: usize = 0;
    let mut two_i: usize = 0;
    while one_i < one.len() || two_i < two.len() {
        if two_i < two.len() && one[one_i] > two[two_i] {
            merged.push(two[two_i]);
            two_i += 1;
            continue;
        }
        merged.push(one[one_i]);
        one_i += 1;
    }
    return merged;
}
