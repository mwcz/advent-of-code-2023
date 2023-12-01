//! A solution to day 1 year 2023.
//! https://adventofcode.com/2023/day/1

use std::{fmt::Display, str::Lines};

type Model = Vec<String>;
type Answer = u32;

pub fn parse(input: String) -> Model {
    input.lines().map(|line| line.to_string()).collect()
}

pub fn part1(input: Model) -> Answer {
    input
        .into_iter()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|v| 10 * v.first().unwrap() + v.last().unwrap())
        .sum()
}

pub fn part2(input: Model) -> Answer {
    input
        .into_iter()
        .map(numberize)
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|v| 10 * v.first().unwrap() + v.last().unwrap())
        .sum()
}

fn numberize(s: String) -> String {
    let mut s = s.to_string();

    let matches = [
        (s.find('1'), false, 1),
        (s.find('2'), false, 2),
        (s.find('3'), false, 3),
        (s.find('4'), false, 4),
        (s.find('5'), false, 5),
        (s.find('6'), false, 6),
        (s.find('7'), false, 7),
        (s.find('8'), false, 8),
        (s.find('9'), false, 9),
        (s.find("one"), true, 1),
        (s.find("two"), true, 2),
        (s.find("three"), true, 3),
        (s.find("four"), true, 4),
        (s.find("five"), true, 5),
        (s.find("six"), true, 6),
        (s.find("seven"), true, 7),
        (s.find("eight"), true, 8),
        (s.find("nine"), true, 9),
        (s.rfind("one"), true, 1),
        (s.rfind("two"), true, 2),
        (s.rfind("three"), true, 3),
        (s.rfind("four"), true, 4),
        (s.rfind("five"), true, 5),
        (s.rfind("six"), true, 6),
        (s.rfind("seven"), true, 7),
        (s.rfind("eight"), true, 8),
        (s.rfind("nine"), true, 9),
    ];

    let mut matches = matches
        .into_iter()
        .filter(|m| m.0.is_some())
        .collect::<Vec<(Option<usize>, bool, u8)>>();

    matches.sort_by(|a, b| a.0.cmp(&b.0));

    let first = matches.first();
    let last = matches.last();

    if first.is_none() && last.is_none() {
        return s;
    }

    for m in [last, first].into_iter().flatten() {
        if let (Some(loc), replace, n) = m {
            let pat = match n {
                1 => "one",
                2 => "two",
                3 => "three",
                4 => "four",
                5 => "five",
                6 => "six",
                7 => "seven",
                8 => "eight",
                9 => "nine",
                _ => "",
            };
            if *replace {
                s.replace_range(*loc..(loc + pat.len()), &n.to_string());
            }
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d1");
    const EXAMPLE: &str = include_str!("../examples/d1");
    const EXAMPLE2: &str = include_str!("../examples/d1-2");

    #[test]
    fn d1p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 142);
    }

    #[test]
    fn d1p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 56108);
    }

    #[test]
    fn d1p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE2.to_string())), 281,);
    }

    #[test]
    fn d1p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 55652,);
    }
}
