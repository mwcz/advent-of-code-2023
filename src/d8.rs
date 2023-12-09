//! A solution to day 8 year 2023.
//! https://adventofcode.com/2023/day/8

use std::{collections::HashMap, time::Duration};

type Model = Map;
type Answer = u32;

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

    println!("starting positions: {:?}", posi);
    let mut steps = 0;
    let mut cycles = vec![vec![0; 0]; posi.len()];

    for mov in model.dirs.iter().cycle() {
        // std::thread::sleep(Duration::from_millis(300));
        // let posi2: Vec<_> = std::mem::take(&mut posi);
        for (i, pos) in posi.iter_mut().enumerate() {
            let dirs = model.nodes.get(*pos).unwrap();
            *pos = match mov {
                Dir::L => &dirs.0,
                Dir::R => &dirs.1,
            };

            if pos.ends_with('Z') {
                cycles[i].push(steps);
            }
        }
        println!("current positions: {:?}", posi);

        steps += 1;

        if posi.iter().all(|node| node.ends_with('Z')) {
            break;
        }
    }

    // let cycle_lens: Vec<Vec<_>> = cycles
    //     .into_iter()
    //     .map(|subg| subg.chunks(2).map(|&[a, b]| b - a))
    //     .collect();
    //

    let first_cycle: Vec<_> = cycles[0].windows(2).map(|pair| pair[1] - pair[0]).collect();
    dbg!(&first_cycle);

    let cycles: Vec<Vec<_>> = cycles
        .iter()
        .map(|cycle| cycle.windows(2).map(|pair| pair[1] - pair[0]).collect())
        .collect();
    dbg!(&cycles);

    steps
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const INPUT: &str = include_str!("../input/d8");
//     const EXAMPLE: &str = include_str!("../examples/d8");
//
//     // #[test]
//     // fn d8p1_example_test() {
//     //     assert_eq!(
//     //         part1(parse(EXAMPLE.to_string())),
//     //         "put part 1 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d8p1_input_test() {
//     //     assert_eq!(
//     //         part1(parse(INPUT.to_string())),
//     //         "put part 1 final answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d8p2_example_test() {
//     //     assert_eq!(
//     //         part2(parse(EXAMPLE.to_string())),
//     //         "put part 2 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d8p2_input_test() {
//     //     assert_eq!(
//     //         part2(parse(INPUT.to_string())),
//     //         "put part 2 final answer here"
//     //     );
//     // }
// }
