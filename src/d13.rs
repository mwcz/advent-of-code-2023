//! A solution to day 13 year 2023.
//! https://adventofcode.com/2023/day/13

use std::fmt::Display;

use crate::grid::Grid;

type Model = Vec<Grid<Ground>>;
type Answer = usize;

pub fn parse(input: String) -> Model {
    input
        .split("\n\n")
        .map(|area| {
            Grid::new(
                area.lines()
                    .map(|line| line.chars().map(Ground::from).collect())
                    .collect(),
            )
        })
        .collect()
}

/// Find all the mirror points in a given row or column.
fn mirrors<T: PartialEq>(cells: &Vec<T>) -> Vec<usize> {
    let mut indexes = vec![];
    for i in 1..(cells.len()) {
        let (left, right) = cells.split_at(i);
        let mut parts_match = true;
        for (j, r) in right.iter().enumerate() {
            if j < left.len() {
                let l = &left[left.len() - j - 1];
                if l != r {
                    parts_match = false;
                }
            } else {
                break;
            }
        }
        if parts_match {
            indexes.push(i);
        }
    }
    indexes
}

fn intersection<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Copy + PartialEq,
{
    a.iter().filter(|x| b.contains(x)).copied().collect()
}

pub fn part1(model: Model) -> Answer {
    let mut sum = 0;
    for area in model {
        // check columns

        let col_mirror = area
            .cols()
            .iter()
            .map(mirrors)
            .reduce(|a, b| intersection(a.as_slice(), b.as_slice()));

        sum += 100 * col_mirror.as_ref().and_then(|m| m.first()).unwrap_or(&0);

        // check rows

        let row_mirror = area
            .rows()
            .iter()
            .map(mirrors)
            .reduce(|a, b| intersection(a.as_slice(), b.as_slice()));

        sum += row_mirror.as_ref().and_then(|m| m.first()).unwrap_or(&0);
    }
    sum
}

pub fn part2(model: Model) -> Answer {
    let mut sum = 0;
    for area in model {
        // check columns

        let col_mirrors: Vec<Vec<usize>> = area.cols().iter().map(mirrors).collect();

        let mut col_counts = vec![0; area.height()];
        for set in &col_mirrors {
            for i in set.iter() {
                col_counts[*i] += 1;
            }
        }
        col_counts.sort();
        let new_col = col_counts[col_counts.len() - 2];
        sum += 100 * new_col;

        let row_mirrors: Vec<Vec<usize>> = area.rows().iter().map(mirrors).collect();

        let mut row_counts = vec![0; area.width()];
        for set in &row_mirrors {
            for i in set.iter() {
                row_counts[*i] += 1;
            }
        }
        row_counts.sort();
        let new_row = row_counts[row_counts.len() - 2];
        sum += new_row;

        println!("col {new_col} row {new_row}");
    }
    sum
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Ground {
    Ash,
    Rock,
}

impl Display for Ground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ground::Ash => write!(f, ".")?,
            Ground::Rock => write!(f, "#")?,
        }
        Ok(())
    }
}

impl From<char> for Ground {
    fn from(value: char) -> Self {
        match value {
            '.' => Ground::Ash,
            '#' => Ground::Rock,
            _ => unreachable!(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const INPUT: &str = include_str!("../input/d13");
//     const EXAMPLE: &str = include_str!("../examples/d13");
//
//     // #[test]
//     // fn d13p1_example_test() {
//     //     assert_eq!(
//     //         part1(parse(EXAMPLE.to_string())),
//     //         "put part 1 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d13p1_input_test() {
//     //     assert_eq!(
//     //         part1(parse(INPUT.to_string())),
//     //         "put part 1 final answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d13p2_example_test() {
//     //     assert_eq!(
//     //         part2(parse(EXAMPLE.to_string())),
//     //         "put part 2 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d13p2_input_test() {
//     //     assert_eq!(
//     //         part2(parse(INPUT.to_string())),
//     //         "put part 2 final answer here"
//     //     );
//     // }
// }
