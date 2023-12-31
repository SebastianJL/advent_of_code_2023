use std::fs;

fn main() {
    let input = "inputs/day_01.txt";
    let contents = fs::read_to_string(input).unwrap();

    let sum: u64 = contents
        .lines()
        .map(|line| {
            let mut digits = vec![];
            for c in line.chars() {
                if c.is_ascii_digit() {
                    digits.push(c);
                }
            }
            let mut num = String::new();
            num.push(digits[0]);
            num.push(digits[digits.len() - 1]);
            let num: u64 = num.parse().unwrap();
            num
        })
        .sum();
    dbg!(sum);
}
