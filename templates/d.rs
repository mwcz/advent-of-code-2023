//! A solution to day $AOC_DAY year $AOC_YEAR.
//! https://adventofcode.com/$AOC_YEAR/day/$AOC_DAY

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

    const INPUT: &str = include_str!("../input/d$AOC_DAY");
    const EXAMPLE: &str = include_str!("../examples/d$AOC_DAY");

    #[test]
    fn d${AOC_DAY}p1_example_test() {
        assert_eq!(
            part1(parse(EXAMPLE.to_string())),
            "put part 1 example answer here"
        );
    }

    #[test]
    fn d${AOC_DAY}p1_input_test() {
        assert_eq!(
            part1(parse(INPUT.to_string())), 
            "put part 1 final answer here"
        );
    }

    #[test]
    fn d${AOC_DAY}p2_example_test() {
        assert_eq!(
            part2(parse(EXAMPLE.to_string())),
            "put part 2 example answer here"
        );
    }

    #[test]
    fn d${AOC_DAY}p2_input_test() {
        assert_eq!(
            part2(parse(INPUT.to_string())),
            "put part 2 final answer here"
        );
    }
}
