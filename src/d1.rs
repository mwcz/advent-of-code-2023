//! A solution to day 1 year 2023.
//! https://adventofcode.com/2023/day/1

type Model = Vec<String>;
type Answer = usize;

pub fn parse(input: String) -> Model {
    input.lines().map(|line| line.to_string()).collect()
}

pub fn part1(model: Model) -> Answer {
    model.into_iter().map(digitize).sum()
}

pub fn part2(model: Model) -> usize {
    model.into_iter().map(numberize).sum()
}

const NAMES: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn digitize(s: String) -> usize {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    (10 * digits.first().unwrap() + digits.last().unwrap()) as usize
}

fn numberize(line: String) -> usize {
    let nums = NAMES.iter().enumerate().chain(DIGITS.iter().enumerate());

    let (_, first) = nums
        .clone()
        .filter_map(|(i, &n)| line.find(n).map(|loc| (loc, i + 1)))
        .reduce(|a, b| if a.0 < b.0 { a } else { b })
        .unwrap();

    let (_, last) = nums
        .filter_map(|(i, &n)| line.rfind(n).map(|loc| (loc, i + 1)))
        .reduce(|a, b| if a.0 > b.0 { a } else { b })
        .unwrap();

    first * 10 + last
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
