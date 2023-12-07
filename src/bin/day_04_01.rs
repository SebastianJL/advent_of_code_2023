use nom::bytes::complete::tag;
use nom::character::complete::{newline, space1, u32};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;
use std::collections::HashSet;
use std::fs;

struct Card {
    _id: u32,
    winning: HashSet<u32>,
    have: HashSet<u32>,
}

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, u32)(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, _id) = preceded(tuple((tag("Card"), space1)), u32)(input)?;
    let (input, _) = tuple((tag(":"), space1))(input)?;
    let (input, (winning, have)) =
        separated_pair(numbers, tuple((tag(" |"), space1)), numbers)(input)?;
    let winning = HashSet::from_iter(winning);
    let have = HashSet::from_iter(have);
    Ok((input, Card { _id, winning, have }))
}

fn parse_cards(input: &str) -> Vec<Card> {
    let (input, cards) = separated_list1(newline, card)(input).unwrap();
    dbg!(input);
    assert!(input.is_empty());
    cards
}
fn main() {
    let input = "inputs/day_04.txt";
    let contents = fs::read_to_string(input).unwrap();

    let mut sum: u32 = 0;
    for card in parse_cards(&contents) {
        let n = card.winning.intersection(&card.have).count() as u32;
        if n >= 1 {
            sum += 2_u32.pow(n - 1);
        }
    }
    dbg!(sum);
}
