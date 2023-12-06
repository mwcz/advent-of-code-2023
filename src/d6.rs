//! A solution to day 6 year 2023.
//! https://adventofcode.com/2023/day/6

type Model = String;
type Answer = u64;

pub fn parse(input: String) -> Model {
    // do part-specific parsing
    input
}

pub fn part1(input: Model) -> Answer {
    let (times, dists) = input.split_once('\n').unwrap();
    let times: Vec<u64> = times[5..]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let dists: Vec<u64> = dists[9..]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let model: Vec<(u64, u64)> = times.into_iter().zip(dists).collect();

    // done parsing

    let mut ans = 1;

    for race in model {
        let mut wins = 0;
        let dur = race.0;
        let record = race.1;

        for wait in 1..dur {
            if wait * (dur - wait) > record {
                wins += 1;
            }
        }
        ans *= wins;
    }

    ans
}

pub fn part2(model: Model) -> Answer {
    let (times, dists) = model.split_once('\n').unwrap();
    let time: u64 = times[9..].trim().replace(' ', "").parse().unwrap();
    let rec: u64 = dists[9..].trim().replace(' ', "").parse().unwrap();

    let mut wins = 0;

    for wait in 1..time {
        if wait * (time - wait) > rec {
            wins += 1;
        }
    }

    wins
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d6");
    const EXAMPLE: &str = include_str!("../examples/d6");

    #[test]
    fn d6p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 288);
    }

    #[test]
    fn d6p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 1159152);
    }

    #[test]
    fn d6p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 71503);
    }

    #[test]
    fn d6p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 41513103);
    }
}
