use std::fs;


fn main() {
    let input = "inputs/day_01.txt";
    let contents = fs::read_to_string(input).unwrap();

    let sum: u64 = contents
        .lines()
        .map(|line| {
            let mut digits = vec![];
            let mut buffer = String::new();

            // forward
            for c in line.chars() {
                if c.is_digit(10) {
                    digits.push(c);
                    break;
                } else {
                    buffer.push(c);
                    if let Some(num) = match "" {
                        _ if buffer.contains("one") => Some('1'),
                        _ if buffer.contains("two") => Some('2'),
                        _ if buffer.contains("three") => Some('3'),
                        _ if buffer.contains("four") => Some('4'),
                        _ if buffer.contains("five") => Some('5'),
                        _ if buffer.contains("six") => Some('6'),
                        _ if buffer.contains("seven") => Some('7'),
                        _ if buffer.contains("eight") => Some('8'),
                        _ if buffer.contains("nine") => Some('9'),
                        _ => None,
                    } {
                        digits.push(num);
                        break;
                    }
                }
            }
            buffer.clear();
            // backward
            for c in line.chars().rev() {
                if c.is_digit(10) {
                    digits.push(c);
                    break;
                } else {
                    buffer.push(c);
                    if let Some(num) = match "" {
                        _ if buffer.contains("eno") => Some('1'),
                        _ if buffer.contains("owt") => Some('2'),
                        _ if buffer.contains("eerht") => Some('3'),
                        _ if buffer.contains("ruof") => Some('4'),
                        _ if buffer.contains("evif") => Some('5'),
                        _ if buffer.contains("xis") => Some('6'),
                        _ if buffer.contains("neves") => Some('7'),
                        _ if buffer.contains("thgie") => Some('8'),
                        _ if buffer.contains("enin") => Some('9'),
                        _ => None,
                    } {
                        digits.push(num);
                        break;
                    }
                }
            }
            let mut num = String::new();
            num.push(digits[0]);
            num.push(digits[digits.len() - 1]);
            let num: u64 = num.parse().unwrap();
            (num, line)
        })
        .inspect(|numline| println!("{numline:?}"))
        .map(|(num, line)| num)
        .sum();
    dbg!(sum);
}
