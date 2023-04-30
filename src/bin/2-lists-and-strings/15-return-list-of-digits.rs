// 15. Write a function that takes a number and returns a list of its digits. 
//  So for 2342 it should return [2,3,4,2].

fn main() {
    let mut num = 42069;
	println!("result: {:?}", itol(num));
}

fn itol(num: i64) -> Vec::<i64> {
	let mut list = Vec::<i64>::new(); 
	let mut i = 1;
	let mut scale = i * 10;
	loop {
		if num/(scale / 10) < 1  {
			break
		}
		list.push((num % scale)/(scale/10));
		num -= num % scale;
		i += 1;
		scale *= 10;
    }
	return list.reverse();
}