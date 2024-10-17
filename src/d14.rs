//! A solution to day 14 year 2023.
//! https://adventofcode.com/2023/day/14

use std::fmt::Display;

use crate::{direction::CardDir, grid::Grid};

type Model = Platform;
type Answer = usize;

#[derive(Debug, Clone)]
pub struct Platform {
    grid: Grid<Rock>,
}

impl Platform {
    fn roll(&mut self, dir: CardDir) {
        let grid_scratch = self.grid.clone();
        match dir {
            CardDir::Up => {
                for (i, col) in grid_scratch.cols().iter().enumerate() {
                    let new_col = roll_vec(col, dir);
                    self.grid.set_col(i, &new_col);
                }
            }
            CardDir::Down => {
                for (i, col) in grid_scratch.cols().iter().enumerate() {
                    let new_col = roll_vec(col, dir);
                    self.grid.set_col(i, &new_col);
                }
            }
            CardDir::Left => {
                for (i, row) in grid_scratch.rows().iter().enumerate() {
                    let new_row = roll_vec(row, dir);
                    self.grid.set_row(i, &new_row);
                }
            }
            CardDir::Right => {
                for (i, row) in grid_scratch.rows().iter().enumerate() {
                    let new_row = roll_vec(row, dir);
                    self.grid.set_row(i, &new_row);
                }
            }
        }
    }

    fn score(&self) -> usize {
        let mut score = 0;

        for (y, row) in self.grid.cells.iter().enumerate() {
            for rock in row.iter() {
                if *rock == Rock::Round {
                    score += self.grid.height() - y;
                }
            }
        }

        score
    }
}

fn roll_vec(v: &[Rock], dir: CardDir) -> Vec<Rock> {
    let mut v = Vec::from(v);
    let dynamic_rocks = [Rock::Round, Rock::Empty];
    let grouped = v.chunk_by_mut(|a, b| dynamic_rocks.contains(a) && dynamic_rocks.contains(b));
    for group in grouped {
        group.sort();
        if [CardDir::Right, CardDir::Down].contains(&dir) {
            group.reverse()
        }
    }
    v
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Rock {
    Round,
    Cube,
    Empty,
}

impl From<char> for Rock {
    fn from(value: char) -> Self {
        match value {
            'O' => Rock::Round,
            '#' => Rock::Cube,
            '.' => Rock::Empty,
            _ => unreachable!(),
        }
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rock::Round => 'O',
                Rock::Cube => '#',
                Rock::Empty => '.',
            }
        )
    }
}

pub fn parse(input: String) -> Model {
    Platform {
        grid: Grid::new(
            input
                .lines()
                .map(|line| line.chars().map(Rock::from).collect())
                .collect(),
        ),
    }
}

pub fn part1(mut model: Model) -> Answer {
    model.roll(CardDir::Up);
    model.score()
}

pub fn part2(mut model: Model) -> Answer {
    let mut dirs = [CardDir::Up, CardDir::Left, CardDir::Down, CardDir::Right]
        .iter()
        .cycle();

    // seed a cycle detector with guaranteed invalid values
    let min_cycle_size = 10;
    let max_cycle_size = 500;
    let mut seq = vec![0; max_cycle_size];
    assert!(seq.len() % 2 == 0);

    let mut cycle = vec![];
    let mut cycle_at = 0;
    let total_cycles = 1000000000;
    // for i in 1..=10 {
    'outer: for i in 1..=total_cycles {
        // apply all four directions
        for _ in 0..4 {
            model.roll(*dirs.next().unwrap());
        }

        seq.rotate_left(1);
        if let Some(last) = seq.last_mut() {
            *last = model.score();
        }

        // split seq into groups of 4 elements
        seq.reverse();

        for j in min_cycle_size..=(seq.len() / 2) {
            let chunks: Vec<&[usize]> = seq.chunks(j).collect();
            if chunks[0] == chunks[1] && i >= max_cycle_size {
                cycle = Vec::from(chunks[0]);
                cycle_at = i;
                break 'outer;
            }
        }

        seq.reverse();
    }
    cycle.reverse();
    cycle[(total_cycles - cycle_at - 1) % cycle.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d14");
    const EXAMPLE: &str = include_str!("../examples/d14");

    #[test]
    fn d14p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 136);
    }

    #[test]
    fn d14p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 105623);
    }

    #[test]
    fn d14p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 64);
    }

    #[test]
    fn d14p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 98029);
    }
}
