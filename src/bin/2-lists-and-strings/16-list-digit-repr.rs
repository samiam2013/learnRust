/* 16. Write functions that add, subtract, and multiply two numbers in their digit-list representation
 (and return a new digit list). 
 If you’re ambitious you can implement Karatsuba multiplication. 
 Try different bases. What is the best base if you care about speed? 
 If you couldn’t completely solve the prime number exercise above due to the lack of large numbers 
 in your language, you can now use your own library for this task. */


fn main() {
    
}

// copy from exercise 2.15

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