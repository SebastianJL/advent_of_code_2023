use advent_of_code_2023::nom::{numbers_u64, separated_triple};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, newline, space1, u64};
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded};
use nom::{IResult, Parser};
use std::fs;
use std::ops::Range;

#[derive(Debug)]
struct ResourceMap {
    source: Range<u64>,
    dest_start: u64,
}

fn seed_list(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("seeds: "), numbers_u64)(input)
}

fn resource_map(input: &str) -> IResult<&str, ResourceMap> {
    let (input, (dest_start, source_start, length)) =
        separated_triple(space1, u64, u64, u64)(input)?;
    Ok((
        input,
        ResourceMap {
            source: source_start..(source_start + length),
            dest_start,
        },
    ))
}

fn resource_maps(input: &str) -> IResult<&str, Vec<ResourceMap>> {
    let (input, _first_line) = alpha1
        .and(tag("-to-"))
        .and(alpha1)
        .and(tag(" map:"))
        .and(newline)
        .parse(input)?;
    separated_list1(newline, resource_map)(input)
}
fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<Vec<ResourceMap>>)> {
    let (input, seeds) = seed_list(input)?;
    let double_newline = || pair(line_ending, line_ending);
    let (input, _) = double_newline()(input)?;
    let (input, resource_maps) = separated_list1(double_newline(), resource_maps)(input)?;
    Ok((input, (seeds, resource_maps)))
}
fn main() {
    let input = "inputs/day_05.txt";
    let contents = fs::read_to_string(input).unwrap();

    let res = parse_input(&contents);
    dbg!(&res);
    let mut min = u64::MAX;
    if let Ok((input, (seeds, map_of_maps))) = res {
        assert!(input.is_empty());
        for seed in seeds {
            let mut val = seed;
            for map in &map_of_maps {
                for r in map {
                    if r.source.contains(&val) {
                        val = r.dest_start + (val - r.source.start);
                        break;
                    }
                }
            }
            if val < min {
                min = val;
            }
        }
        dbg!(min);
    }
}
