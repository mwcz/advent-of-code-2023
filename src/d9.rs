//! A solution to day 9 year 2023.
//! https://adventofcode.com/2023/day/9

type Model = Vec<Vec<i32>>;
type Answer = i32;

pub fn parse(input: String) -> Model {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|ns| ns.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(layers: Model) -> Answer {
    let mut sum: i32 = 0;

    for layer in &layers {
        let diffs = diff_until_zero(layer);
        let mut xx = vec![0; diffs.len()];

        let mut xprev = 0;
        for (i, x) in xx[1..].iter_mut().enumerate().rev() {
            *x = xprev + diffs[i].last().unwrap();
            xprev = *x;
        }

        sum += xprev;
    }

    sum
}

pub fn part2(mut layers: Model) -> Answer {
    for layer in layers.iter_mut() {
        layer.reverse();
    }

    part1(layers)
}

fn diff_until_zero(seq: &[i32]) -> Vec<Vec<i32>> {
    let mut layers = vec![];
    let mut layer = seq.to_vec();

    loop {
        layers.push(layer.to_vec());

        if all_zero(&layer) {
            break;
        }

        layer = diff(&layer);
    }

    layers
}

fn diff(seq: &[i32]) -> Vec<i32> {
    seq.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

fn all_zero(seq: &[i32]) -> bool {
    seq.iter().all(|&n| n == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d9");
    const EXAMPLE: &str = include_str!("../examples/d9");

    #[test]
    fn d9p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 114);
    }

    #[test]
    fn d9p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 1853145119);
    }

    #[test]
    fn d9p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 2);
    }

    #[test]
    fn d9p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 923);
    }
}
