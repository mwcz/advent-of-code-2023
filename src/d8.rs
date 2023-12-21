//! A solution to day 8 year 2023.
//! https://adventofcode.com/2023/day/8

use std::collections::HashMap;

type Model = Map;
type Answer = usize;

const START: &str = "AAA";
const END: &str = "ZZZ";

pub fn parse(input: String) -> Model {
    let (dirs, nodes) = input.split_once("\n\n").unwrap();

    let dirs = dirs.chars().map(Dir::from).collect();

    let nodes = nodes
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" = ").unwrap();
            (
                from.to_string(),
                (to[1..4].to_string(), to[6..9].to_string()),
            )
        })
        .collect();

    Map { dirs, nodes }
}

pub fn part1(model: Model) -> Answer {
    dbg!(&model);

    let mut pos = START;
    let mut steps = 0;
    for mov in model.dirs.iter().cycle() {
        let dirs = model.nodes.get(pos).unwrap();
        pos = match mov {
            Dir::L => &dirs.0,
            Dir::R => &dirs.1,
        };

        steps += 1;

        if pos == END {
            break;
        }
    }

    steps
}

pub fn part2(model: Model) -> Answer {
    let mut posi: Vec<_> = model
        .nodes
        .iter()
        .filter(|c| c.0.ends_with('A'))
        .map(|c| c.0)
        .collect();

    let mut cycles = vec![0; posi.len()];
    let last_zs = vec![0; posi.len()];

    for (i, mov) in model.dirs.iter().cycle().enumerate() {
        let step = i + 1;
        // std::thread::sleep(Duration::from_millis(300));
        // let posi2: Vec<_> = std::mem::take(&mut posi);
        for (i, pos) in posi.iter_mut().enumerate() {
            let dirs = model.nodes.get(*pos).unwrap();
            *pos = match mov {
                Dir::L => &dirs.0,
                Dir::R => &dirs.1,
            };

            if pos.ends_with('Z') {
                let last_z = last_zs[i];
                let this_cycle = step - last_z;
                let last_cycle = cycles[i];
                if last_cycle != this_cycle {
                    cycles[i] = this_cycle;
                }
            }
        }

        // if all cycles are nonzero, we're done
        if cycles.iter().all(|&c| c != 0) {
            break;
        }
    }

    cycles.into_iter().reduce(lcm).unwrap()
}

#[derive(Debug)]
pub struct Map {
    dirs: Vec<Dir>,
    nodes: HashMap<String, (String, String)>,
}

#[derive(Debug)]
enum Dir {
    L,
    R,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'R' => Dir::R,
            'L' => Dir::L,
            _ => panic!("invalid direction: {value}"),
        }
    }
}

pub fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while a % b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    b
}

pub fn lcm(a: usize, b: usize) -> usize {
    let g = gcd(a, b);
    (a * b) / g
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d8");
    const EXAMPLE1: &str = include_str!("../examples/d8");
    const EXAMPLE2: &str = include_str!("../examples/d8-2");
    const EXAMPLE3: &str = include_str!("../examples/d8-3");

    #[test]
    fn d8p1_example_1_test() {
        assert_eq!(part1(parse(EXAMPLE1.to_string())), 2);
    }

    #[test]
    fn d8p1_example_2_test() {
        assert_eq!(part1(parse(EXAMPLE2.to_string())), 6);
    }

    #[test]
    fn d8p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 17263);
    }

    #[test]
    fn d8p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE3.to_string())), 6);
    }

    #[test]
    fn d8p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 14631604759649);
    }
}
