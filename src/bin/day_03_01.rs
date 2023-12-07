use itertools::Itertools;
use ndarray::{Array2, Axis};
use std::fs;

const SYMBOL_CHAR: char = '?';

fn main() {
    let input = "inputs/day_03.txt";
    let contents = fs::read_to_string(input).unwrap();
    let ncols = contents.lines().next().unwrap().len();
    let nrows = contents.lines().count();

    let chars = contents
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| {
                if c.is_ascii_digit() || c == '.' {
                    c
                } else {
                    SYMBOL_CHAR // mark every symbol with SYMBOL_CHAR for easier processing later.
                }
            })
        })
        .collect_vec();
    let engine_schematic = Array2::from_shape_vec((nrows, ncols), chars).unwrap();

    let mut sum = 0;

    for (i, row) in engine_schematic.axis_iter(Axis(0)).enumerate() {
        let mut num = 0;
        let mut marked = false;
        for (j, &c) in row.iter().enumerate() {
            if !c.is_ascii_digit() {
                if marked {
                    sum += num;
                }
                marked = false;
                num = 0;
                continue;
            }

            num *= 10;
            num += c.to_digit(10).unwrap();

            let (u, v) = (i as isize, j as isize);
            let checked_indices = [
                (u - 1, v),
                (u - 1, v + 1),
                (u, v + 1),
                (u + 1, v + 1),
                (u + 1, v),
                (u + 1, v - 1),
                (u, v - 1),
                (u - 1, v - 1),
            ]
            .into_iter()
            .filter(|&(u, v)| u >= 0 && v >= 0 && u < nrows as isize && v < ncols as isize)
            .map(|(u, v)| (u as usize, v as usize));

            if !marked {
                let num_marked_cells = checked_indices
                    .flat_map(|index| engine_schematic.get(index))
                    .flat_map(|&c| if c == SYMBOL_CHAR { Some(c) } else { None })
                    .count();
                if num_marked_cells > 0 {
                    marked = true;
                }
            }
        }
        if marked {
            sum += num;
        }
    }
    dbg!(sum);
}
