//! A solution to day 3 year 2023.
//! https://adventofcode.com/2023/day/3

use std::collections::HashMap;

type Model = Vec<Vec<char>>;
type Answer = u32;

pub fn parse(input: String) -> Model {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part1(model: Model) -> Answer {
    fn is_symbol(c: &char) -> bool {
        !c.is_ascii_digit() && c != &'.'
    }

    let is_symbol_adjacent = |x: usize, y: usize| {
        [
            (x.saturating_sub(1), y.saturating_sub(1)),
            (x, y.saturating_sub(1)),
            (x + 1, y.saturating_sub(1)),
            (x.saturating_sub(1), y),
            (x + 1, y),
            (x.saturating_sub(1), y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .map(|(x, y)| model.get(y).and_then(|row| row.get(x)))
        .into_iter()
        .flatten()
        .any(is_symbol)
    };

    let mut part_nums: Vec<u32> = vec![];

    for (y, row) in model.iter().enumerate() {
        let mut digits = vec![];
        let mut is_part_num = false;
        for (x, c) in row.iter().enumerate() {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if is_symbol_adjacent(x, y) {
                        is_part_num = true;
                    }
                    digits.push(c)
                }
                _ => {
                    if !digits.is_empty() && is_part_num {
                        let n = to_number(&digits);

                        part_nums.push(n);
                    }

                    // number ended, clear this flag
                    is_part_num = false;
                    digits.clear();
                }
            }
        }

        if !digits.is_empty() && is_part_num {
            let n = to_number(&digits);

            part_nums.push(n);
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
    let get_adj_gear = |x: usize, y: usize| -> Option<(usize, usize)> {
        [
            (x.saturating_sub(1), y.saturating_sub(1)),
            (x, y.saturating_sub(1)),
            (x + 1, y.saturating_sub(1)),
            (x.saturating_sub(1), y),
            (x + 1, y),
            (x.saturating_sub(1), y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .map(|(x, y)| ((x, y), model.get(y).and_then(|row| row.get(x))))
        .into_iter()
        .filter(|m| m.1.is_some() && m.1.unwrap() == &'*')
        .map(|m| m.0)
        .next()
    };

    let mut gear_ratios: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for (y, row) in model.iter().enumerate() {
        let mut digits = vec![];
        let mut gear_loc: Option<(usize, usize)> = None;
        for (x, c) in row.iter().enumerate() {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if let Some((x, y)) = get_adj_gear(x, y) {
                        gear_loc = Some((x, y));
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
