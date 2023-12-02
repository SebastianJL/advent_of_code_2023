use std::fs;
use std::iter::Sum;
use std::ops::Add;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1, u64};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Default, Debug)]
struct CubeSet {
    red: u64,
    green: u64,
    blue: u64,
}

impl CubeSet {
    fn contains(&self, other: CubeSet) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

impl Add for CubeSet {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
        self
    }
}

impl Sum for CubeSet {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(CubeSet::default(), |a, b| a + b)
    }
}

#[derive(Debug)]
struct Game {
    id: u64,
    draws: Vec<CubeSet>,
}

fn color(input: &str) -> IResult<&str, Color> {
    let (input, color) = alt((tag("red"), tag("green"), tag("blue")))(input)?;
    match color {
        "red" => Ok((input, Color::Red)),
        "green" => Ok((input, Color::Green)),
        "blue" => Ok((input, Color::Blue)),
        _ => unreachable!(),
    }
}

fn number_and_color(input: &str) -> IResult<&str, CubeSet> {
    let (input, number) = u64(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = color(input)?;
    let res = match color {
        Color::Red => CubeSet {
            red: number,
            ..Default::default()
        },
        Color::Green => CubeSet {
            green: number,
            ..Default::default()
        },
        Color::Blue => CubeSet {
            blue: number,
            ..Default::default()
        },
    };
    Ok((input, res))
}

fn draw(input: &str) -> IResult<&str, CubeSet> {
    let (input, draws) = separated_list1(tag(", "), number_and_color)(input)?;
    Ok((input, draws.into_iter().sum()))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, (_, id, _)) = tuple((tag("Game "), u64, tag(": ")))(input)?;
    let (input, draws) = separated_list1(tag("; "), draw)(input)?;
    let res = Game { id, draws };
    Ok((input, res))
}

fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, game)(input)
}

fn main() {
    let input = "inputs/day_02.txt";
    let contents = fs::read_to_string(input).unwrap();

    let (input, games) = parse(&contents).unwrap();
    assert!(input.is_empty());

    dbg!(&games);

    let mut minimal_sets = vec![];

    for game in games {
        let mut min_set = CubeSet::default();
        for draw in game.draws {
            if draw.red > min_set.red {
                min_set.red = draw.red
            }
            if draw.green > min_set.green {
                min_set.green = draw.green
            }
            if draw.blue > min_set.blue {
                min_set.blue = draw.blue
            }
        }
        minimal_sets.push(min_set.power());
    }

    dbg!(&minimal_sets);

    let res: u64 = minimal_sets.into_iter().sum();
    dbg!(res);
}
