//! A solution to day 10 year 2023.
//! https://adventofcode.com/2023/day/10

use std::fmt::Display;

use crate::grid::Grid;

type Model = Grid<Pipe>;
type Answer = u32;

#[derive(Copy, Clone)]
pub enum Pipe {
    Start,
    Vert,
    Horiz,
    TL,
    TR,
    BL,
    BR,
    None,
}

impl Pipe {
    fn from_char(c: char) -> Pipe {
        match c {
            'S' => Pipe::Start,
            '-' => Pipe::Horiz,
            '|' => Pipe::Vert,
            'L' => Pipe::BL,
            'J' => Pipe::BR,
            'F' => Pipe::TL,
            '7' => Pipe::TR,
            _ => Pipe::None,
        }
    }
    fn to_char(p: &Pipe) -> char {
        match p {
            Pipe::Start => 'S',
            Pipe::Vert => '║',
            Pipe::Horiz => '═',
            Pipe::TL => '╔',
            Pipe::TR => '╗',
            Pipe::BL => '╚',
            Pipe::BR => '╝',
            Pipe::None => ' ',
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Pipe::to_char(self))
    }
}

pub fn parse(input: String) -> Model {
    let g = Grid::<Pipe>::new(
        input
            .lines()
            .map(|line| line.chars().map(Pipe::from_char).collect())
            .collect(),
    );
    g
}

pub fn part1(model: Model) -> Answer {
    println!("{model}");
    todo!();
}

pub fn part2(model: Model) -> Answer {
    todo!();
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const INPUT: &str = include_str!("../input/d10");
//     const EXAMPLE: &str = include_str!("../examples/d10");
//
//     // #[test]
//     // fn d10p1_example_test() {
//     //     assert_eq!(
//     //         part1(parse(EXAMPLE.to_string())),
//     //         "put part 1 example answer here"
//     //     );
//     // }
//
//     // #[test]
//     // fn d10p1_input_test() {
//     //     assert_eq!(
//     //         part1(parse(INPUT.to_string())),
//     //         "put part 1 final answer here"
//     //     );
//     // }
//
//     // #[test]
//     // fn d10p2_example_test() {
//     //     assert_eq!(
//     //         part2(parse(EXAMPLE.to_string())),
//     //         "put part 2 example answer here"
//     //     );
//     // }
//
//     // #[test]
//     // fn d10p2_input_test() {
//     //     assert_eq!(
//     //         part2(parse(INPUT.to_string())),
//     //         "put part 2 final answer here"
//     //     );
//     // }
//
// }
