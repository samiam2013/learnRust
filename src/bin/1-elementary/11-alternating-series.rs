// 11. Write a program that computes the sum of an alternating series where each element of the series
// is an expression of the form ((-1)^{k+1})/(2 * k-1) for each value of k from 1 to a million, multiplied by 4.d

fn main() {
    let mut sum = 0.0;
    for k in 1..=1_000_000 {
        sum += ((-1.0_f64).powi(k as i32 + 1)) / (2.0 * k as f64 - 1.0);
    }
    println!("{:}", sum * 4.0);
}
