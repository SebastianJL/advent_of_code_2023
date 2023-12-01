use nom::character::complete::{alpha1, line_ending};
use nom::multi::separated_list0;
use std::fs;
use regex::Regex;

fn main() {
    let input = "inputs/day_01_02.txt";
    let contents = fs::read_to_string(input).unwrap();

    // let re = Regex::new("([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    // let search = "4twone";
    // for c in re.find_iter(search) {
    //     dbg!(search[c.0..=c.1]);
    // }

    let sum: u64 = contents
        .lines()
        .map(|line| {
            let mut digits: Vec<char> = vec![];
            let re = Regex::new("([0-9]|one|two|three|four|five|six|seven|eight|nine)");
            for c in re.unwrap().captures_iter(line) {
                let num = c.get(1).unwrap().as_str();
                if num.chars().next().unwrap().is_ascii_digit() {
                    digits.push(num.chars().next().unwrap());
                } else {
                    match num {
                        "one" => digits.push('1'),
                        "two" => digits.push('2'),
                        "three" => digits.push('3'),
                        "four" => digits.push('4'),
                        "five" => digits.push('5'),
                        "six" => digits.push('6'),
                        "seven" => digits.push('7'),
                        "eight" => digits.push('8'),
                        "nine" => digits.push('9'),
                        _ => unreachable!(),
                    }
                }
            }
            let mut numstr = String::new();
            numstr.push(digits[0]);
            numstr.push(digits[digits.len() - 1]);
            let num: u64 = numstr.parse().unwrap();
            (num, line)
        })
        .inspect(|numline| {
            dbg!(&numline);
        })
        .map(|(num, line)| num)
        .sum();
    dbg!(sum);
}
