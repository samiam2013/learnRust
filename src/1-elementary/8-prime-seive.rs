use std::io::stdin; 
use std::io::{self, Write};

fn main(){
    let mut n_input = String::new();
    print!("find n primes: ");
    io::stdout().flush().unwrap();
    stdin().read_line(&mut n_input).unwrap();
    n_input.truncate(n_input.trim().len());
    let n = n_input.parse::<i64>().unwrap();
    println!("finding {:} primes", n);
    
    let mut candidate: i64 = 3;
    let mut found_cnt: i64 = 0;
    while found_cnt < n {
        let cand_float = candidate as f64;
        let cand_sqrt = cand_float.sqrt();
        let ceil = cand_sqrt.ceil() as i64;
        let mut divisor = 2; 
        let mut is_prime = true; 
        while divisor < (ceil + (ceil % 2) + 1) {
            if candidate % divisor == 0 {
                is_prime = false;
            }
            divisor += 1; 
        }
        if is_prime {
            println!("prime found {:}", candidate);
            found_cnt += 18
            ;
        }
        candidate += 2;
    }
}