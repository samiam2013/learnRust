// Modify the previous program such that only multiples of three or five
//  are considered in the sum, e.g. 3, 5, 6, 9, 10, 12, 15 for n=17
use std::io::stdin;
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.truncate(input.trim().len());
    let conv_num = input.parse::<i64>();
    let num_int = conv_num.unwrap();
    println!("converted number: {:?}", num_int);
    let triangle = (num_int * (num_int + 1)) / 2;
    println!("triangle sum: {:?}", triangle);
}
