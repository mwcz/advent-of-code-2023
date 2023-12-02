//! A solution to day 2 year 2023.
//! https://adventofcode.com/2023/day/2

type Model = Vec<Vec<Color>>;
type Answer = u32;

pub fn parse(input: String) -> Model {
    input.lines().map(line_to_color)
}

pub fn part1(model: Model) -> Answer {
    "incomplete".to_string()
}

pub fn part2(model: Model) -> Answer {
    "incomplete".to_string()
}

struct Color {
    r: u32,
    g: u32,
    b: u32,
}

fn line_to_color(line: &str) -> Vec<Color> {
    let mut chars = line.chars();

    chars.take_while(|c| !c.is_digit(10));

    vec![]
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const INPUT: &str = include_str!("../input/d2");
//     const EXAMPLE: &str = include_str!("../examples/d2");
//
//     // #[test]
//     // fn d2p1_example_test() {
//     //     assert_eq!(
//     //         part1(parse(EXAMPLE.to_string())),
//     //         "put part 1 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d2p1_input_test() {
//     //     assert_eq!(
//     //         part1(parse(INPUT.to_string())),
//     //         "put part 1 final answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d2p2_example_test() {
//     //     assert_eq!(
//     //         part2(parse(EXAMPLE.to_string())),
//     //         "put part 2 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d2p2_input_test() {
//     //     assert_eq!(
//     //         part2(parse(INPUT.to_string())),
//     //         "put part 2 final answer here"
//     //     );
//     // }
// }
