//! A solution to day 3 year 2023.
//! https://adventofcode.com/2023/day/3

use crate::{
    grid::{Cell, Grid},
    point::Point,
};
use std::collections::HashMap;

type Model = Grid<char>;
type Answer = u32;

pub fn parse(input: String) -> Model {
    Grid::new(input.lines().map(|line| line.chars().collect()).collect())
}

pub fn part1(model: Model) -> Answer {
    fn is_symbol(c: Cell<char>) -> bool {
        !c.data.is_ascii_digit() && c.data != '.'
    }

    let is_symbol_adjacent = |x: usize, y: usize| {
        //
        model.adj(x, y).cells.into_iter().flatten().any(is_symbol)
    };

    let mut part_nums: Vec<u32> = vec![];

    for (y, row) in model.cells.iter().enumerate() {
        let mut num = 0;
        let mut is_part_num = false;
        for (x, c) in row.iter().enumerate() {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if is_symbol_adjacent(x, y) {
                        is_part_num = true;
                    }
                    num = num * 10 + c.to_digit(10).unwrap();
                }
                _ => {
                    if is_part_num {
                        part_nums.push(num);
                    }

                    // number ended, clear this flag
                    is_part_num = false;
                    num = 0;
                }
            }
        }

        if is_part_num {
            part_nums.push(num);
        }
    }

    part_nums.iter().sum()
}

fn to_number(digits: &[&char]) -> u32 {
    digits
        .iter()
        .filter_map(|c| c.to_digit(10))
        .rev()
        .enumerate()
        .map(|(i, d)| d * 10u32.pow(i as u32))
        .collect::<Vec<u32>>()
        .into_iter()
        .rev()
        .sum()
}

pub fn part2(model: Model) -> Answer {
    let get_adj_gear = |x: usize, y: usize| -> Option<Point<2>> {
        model
            .adj(x, y)
            .cells
            .into_iter()
            .filter_map(|cello| cello.map(|cell| (cell.data == '*').then(|| cell.pos)))
            .flatten()
            .next()
    };

    let mut gear_ratios: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for (y, row) in model.cells.iter().enumerate() {
        let mut digits = vec![];
        let mut gear_loc: Option<(usize, usize)> = None;
        for (x, c) in row.iter().enumerate() {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if let Some(point) = get_adj_gear(x, y) {
                        gear_loc = Some((point.x(), point.y()));
                    }
                    digits.push(c)
                }
                _ => {
                    // turn chars into a number
                    if !digits.is_empty() && gear_loc.is_some() {
                        let n = to_number(&digits);

                        // add entry to hashmap or update
                        let entry = gear_ratios.entry(gear_loc.unwrap()).or_default();
                        entry.push(n);
                    }

                    // number ended, clear this flag
                    gear_loc = None;
                    digits.clear();
                }
            }
        }

        if !digits.is_empty() && gear_loc.is_some() {
            let n = to_number(&digits);
            let entry = gear_ratios.entry(gear_loc.unwrap()).or_default();
            entry.push(n);
        }
    }

    dbg!(&gear_ratios);

    gear_ratios
        .values()
        .filter(|ratios| ratios.len() == 2)
        .inspect(|f| {
            dbg!(&f);
        })
        .map(|ratios| ratios[0] * ratios[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d3");
    const EXAMPLE: &str = include_str!("../examples/d3");

    #[test]
    fn d3p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 4361);
    }

    #[test]
    fn d3p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 509115);
    }

    #[test]
    fn d3p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 467835);
    }

    #[test]
    fn d3p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 75220503);
    }
}
