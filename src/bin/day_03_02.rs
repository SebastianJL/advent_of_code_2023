// use itertools::Itertools;
// use ndarray::{Array2, Axis};
// use std::fs;
//
// const GEAR_CHAR: char = '?';
//
// fn main() {
//     let input = "inputs/day_03.txt";
//     let contents = fs::read_to_string(input).unwrap();
//     let ncols = contents.lines().next().unwrap().len();
//     let nrows = contents.lines().count();
//
//     let chars = contents.lines().flat_map(|line| line.chars()).collect_vec();
//     let engine_schematic = Array2::from_shape_vec((nrows, ncols), chars).unwrap();
//
//     let mut sum = 0;
//
//     for (i, row) in engine_schematic.axis_iter(Axis(0)).enumerate() {
//         for (j, &c) in row.iter().enumerate() {
//             if c == GEAR_CHAR {
//                 let (u, v) = (i as isize, j as isize);
//                 let checked_indices = [
//                     (u - 1, v),
//                     (u - 1, v + 1),
//                     (u, v + 1),
//                     (u + 1, v + 1),
//                     (u + 1, v),
//                     (u + 1, v - 1),
//                     (u, v - 1),
//                     (u - 1, v - 1),
//                 ]
//                 .into_iter()
//                 .filter(|&(u, v)| u >= 0 && v >= 0 && u < nrows as isize && v < ncols as isize)
//                 .map(|(u, v)| (u as usize, v as usize))
//                 .collect_vec();
//
//                 let num_neighbouring_numbers = checked_indices
//                     .iter()
//                     .flat_map(|index| engine_schematic.get(index))
//                     .flat_map(|&c| if c.is_ascii_digit() { Some(c) } else { None })
//                     .count();
//
//                 if num_neighbouring_numbers == 2 {
//                     checked_indices
//                         .flat_map(|index| {
//                             let c = engine_schematic.get(index).unwrap();
//                             1..3
//                         });
//                 }
//             }
//         }
//     }
//     dbg!(sum);
// }

fn main() {}
