use std::fs;
use std::ops::RangeInclusive;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::is_alphanumeric;
use nom::character::complete::{alpha1, char, digit1, line_ending, space1};
use nom::IResult;
use nom::multi::{many1, separated_list0};
use nom::sequence::{separated_pair, tuple};

fn range(i: &str) -> IResult<&str, RangeInclusive<usize>> {
    let (i, (min, max)) = separated_pair(digit1, char('-'), digit1)(i)?;
    let (min, max): (usize, usize) = (min.parse().unwrap(), max.parse().unwrap());
    Ok((i, min..=max))
}

fn marked_indices(i: &str) -> IResult<&str, Vec<usize>> {
    let (input, res) = many1(alt((take_while1(|c| c == 'x'), space1)))(i)?;
    let mut i = 0;
    let mut marked = vec![];
    for r in res {
        if r.starts_with(' ') {
            for j in i..(i + r.len()) {
                marked.push(j);
            }
        }
        i += r.len()
    }
    Ok((input, marked))
}

#[derive(Debug)]
enum File {
    File(String),
    Dir(String),
}
#[derive(Debug)]
enum Command {
    LS(Vec<File>),
    CD(File),
}

fn file_name(input: &str) -> IResult<&str, &str> {
    take_while1(|x: char| x == '.' || is_alphanumeric(x.try_into().unwrap()))(input)
}

fn cd_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$cd ")(input)?;
    let (input, dirname) = file_name(input)?;
    let (input, _) = line_ending(input)?;
    let dir = Command::CD(File::Dir(dirname.to_owned()));
    Ok((input, dir))
}

fn file_line(input: &str) -> IResult<&str, File> {
    let (input, (_, kind, _, name)) = tuple((tag("- "), alpha1, tag(": "), file_name))(input)?;
    let res = match kind {
        "dir" => File::Dir(name.into()),
        "file" => File::File(name.into()),
        &_ => {
            unreachable!()
        }
    };
    Ok((input, res))
}

fn ls_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ls")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, files) = separated_list0(line_ending, file_line)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, Command::LS(files)))
}
fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    many1(alt((ls_command, cd_command)))(input)
}

fn parse_u8(input: &str) -> IResult<&str, u8> {
    nom::character::complete::u8(input)
}

fn main() {
    let input = "inputs/day_01.txt";
    let contents = fs::read_to_string(input).unwrap();

    let res = commands(&contents);
    dbg!(res);
    println!("hello");

    println!("{:?}", parse_u8("255").unwrap());
}
