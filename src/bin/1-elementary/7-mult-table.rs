fn main() {
    for x in 1..13 {
        for y in 1..13 {
            let prod: i64 = x * y;
            print!(" {:>3}", prod);
        }
        println!()
    }
}

