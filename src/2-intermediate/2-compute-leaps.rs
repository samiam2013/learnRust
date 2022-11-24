// 2. Write a program that takes the duration of a year (in fractional days) for an imaginary planet
//  as an input and produces a leap-year rule that minimizes the difference to the planet’s solar year.

fn main() {
    let rules = compute_ly_rule(365.2422);
    for rule in rules {
        println!("{:}", rule);
    }
}

fn compute_ly_rule(days: f64) -> Vec<String> {
    println!("{:}", days);
    let mut extra_day = days - days.floor();
    println!("extra_day: {:?}", extra_day);
    let mut last_freq = 2;
    let mut rules: Vec<String> = Vec::new();
    for i in last_freq..100_000 {
        if i % last_freq != 0 || !(i < 100 || i % 100 == 0) {
            continue;
        }
        if extra_day >= ((1.0 / i as f64) - ((1.0 / i as f64) * 0.1)) {
            rules.push(format!("must have a leap year every {:} years", i));
            extra_day -= 1.0 / i as f64;
            last_freq = i;
        }
        if extra_day <= -((1.0 / i as f64) + ((1.0 / i as f64) * 0.1)) {
            rules.push(format!("must remove a leap year every {:} years", i));
            extra_day += 1.0 / i as f64;
            last_freq = i;
        }
    }
    rules
}
