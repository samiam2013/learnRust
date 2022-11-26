// 10. Write a program that prints the next 20 leap years.
use chrono::Datelike;

fn main() { 
    // get current year as u32
    let mut year = chrono::Local::now().year() as u32;

    let mut count = 0;
    while count < 20 {
        //  if the year is divisible by 100 and not divisible by 400, leap year is skipped
        if year % 100 == 0 && year % 400 != 0 {
            year += 1;
            continue;
        }
        if year % 4 == 0 {
            println!("{:}", year);
            count += 1;
        }
        year += 1;
    }
}

