use std::fs;

fn main() {
    let input = "inputs/day_05.txt";
    let contents = fs::read_to_string(input).unwrap();

    for line in contents.lines() {
        println!("{line}");
    }
}
