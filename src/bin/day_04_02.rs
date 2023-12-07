use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, space1, u32};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct CardSet {
    repeat: u32,
    matches: u32,
}

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, u32)(input)
}

fn card(input: &str) -> IResult<&str, u32> {
    let (input, _id) = preceded(tuple((tag("Card"), space1)), u32)(input)?;
    let (input, _) = tuple((tag(":"), space1))(input)?;
    let (input, (winning, have)) =
        separated_pair(numbers, tuple((tag(" |"), space1)), numbers)(input)?;
    let winning: HashSet<u32> = HashSet::from_iter(winning);
    let have: HashSet<u32> = HashSet::from_iter(have);
    Ok((input, winning.intersection(&have).count() as u32))
}

fn parse_cards(input: &str) -> Vec<u32> {
    let (input, cards) = separated_list1(newline, card)(input).unwrap();
    dbg!(input);
    assert!(input.is_empty());
    cards
}
fn main() {
    let input = "inputs/day_04.txt";
    let contents = fs::read_to_string(input).unwrap();

    let cards = parse_cards(&contents);
    let mut copies = cards
        .into_iter()
        .map(|c| CardSet {
            matches: c,
            repeat: 1,
        })
        .collect_vec();

    for i in 0..copies.len() {
        let matches = copies[i].matches;
        let repeat = copies[i].repeat;
        // for j in i+1..(i + 1 + matches as usize) {
        for copy in copies.iter_mut().skip(i+1).take(matches as usize) {
            copy.repeat += repeat;
        }
    }

    let sum: u32 = copies.iter().map(|c| c.repeat).sum();
    dbg!(copies);
    dbg!(sum);
}
