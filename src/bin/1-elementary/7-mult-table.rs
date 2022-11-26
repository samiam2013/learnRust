fn main() {
    for x in 1..13 {
        for y in 1..13 {
            let prod: i64 = x * y;
            print!(" {:>3}", prod);
        }
        println!()
    }
}

// go version
// func MultiplicationTable() {
// 	for row := 1; row <= 12; row++ {
// 		for column := 1; column <= 12; column++ {
// 			product := row * column
// 			fmt.Printf("%4v", product)
// 		}
// 		fmt.Print("\n")
// 	}
// }
