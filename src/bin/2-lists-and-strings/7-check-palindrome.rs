/* 7. Write a function that tests whether a string is a palindrome. */

fn main() {
    let candidates = vec!("hannah", "racecar", "lol", "palindrome");
    for cand in candidates {
        println!("candidate '{:}' palindrome status: {:} revese() version status: {:}", 
        cand, is_palindrome(cand.to_string()), reverse_is_palindrome(cand.to_string()));
    }
}

fn is_palindrome(candidate: String) -> bool {
    let mut i = 0;
    while i < candidate.len()/2 {
        let opp = candidate.len() - (i + 1); 
        if candidate.chars().nth(i).unwrap() != candidate.chars().nth(opp).unwrap() {
            return false;
        }
        i += 1;
    }
    return true;
}

fn reverse_is_palindrome(candidate: String) -> bool {
    let mut reversed = candidate.as_bytes().to_owned();
    reversed.reverse();
    if reversed == candidate.as_bytes() {
        return true;
    }
    return false;
}