//! A solution to day 12 year 2023.
//! https://adventofcode.com/2023/day/12

type Model = u8;
type Answer = String;

pub fn parse(input: String) -> Model {
    0
}

pub fn part1(input: Model) -> Answer {
    "incomplete".to_string()
}

pub fn part2(input: Model) -> Answer {
    "incomplete".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d12");
    const EXAMPLE: &str = include_str!("../examples/d12");

    #[test]
    fn d12p1_example_test() {
        assert_eq!(
            part1(parse(EXAMPLE.to_string())),
            "put part 1 example answer here"
        );
    }

    #[test]
    fn d12p1_input_test() {
        assert_eq!(
            part1(parse(INPUT.to_string())), 
            "put part 1 final answer here"
        );
    }

    #[test]
    fn d12p2_example_test() {
        assert_eq!(
            part2(parse(EXAMPLE.to_string())),
            "put part 2 example answer here"
        );
    }

    #[test]
    fn d12p2_input_test() {
        assert_eq!(
            part2(parse(INPUT.to_string())),
            "put part 2 final answer here"
        );
    }
}
