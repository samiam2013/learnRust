/* 3. Write function that reverses a list, preferably in place. */ 

fn main() {
    let mut list: Vec<i32> = vec!(1, 4, -3, 19);
    reverse(&mut list);
    println!("{:?}",list);
}

fn reverse(list: &mut Vec<i32>) {
    let mut i = list.len()-1;
    while i >= list.len()/2 {
        println!("replaced {:} with {:}", i,list.len() - (i+1) );
        let opp = list.len()-(i+1);
        let buf = list[opp];
        list[opp] = list[i];
        list[i] = buf; 
        
        i -= 1;
    }
}