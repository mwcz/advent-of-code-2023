//! A solution to day 17 year 2023.
//! https://adventofcode.com/2023/day/17

use crate::{direction::CardDir, grid::Grid, point::Point};
use pathfinding::prelude::astar;

type Model = Grid<usize>;
type Answer = usize;

pub fn parse(input: String) -> Model {
    Grid::new(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect(),
    )
}

pub fn part1(model: Model) -> Answer {
    use CardDir::*;
    let start: Point<2> = [0, 0].into();
    let end: Point<2> = [model.width() - 1, model.height() - 1].into();

    // a "fake" starting direction to force all traversals to begin with a "turn"
    let faux_starting_dir = Up;

    let path = astar(
        &(start, faux_starting_dir, 3),
        |(p, dir, straight_rem)| {
            // tag each adjacent point with its associated direction
            let [up, left, right, down] = model.adj_4(p.x(), p.y()).cells;

            let up = up.map(|p| (p, Up));
            let left = left.map(|p| (p, Left));
            let right = right.map(|p| (p, Right));
            let down = down.map(|p| (p, Down));

            let successors: Vec<_> = [up, left, right, down]
                .into_iter()
                .map(|d| {
                    if let Some(d) = d {
                        let new_straight_rem = match (dir, d.1) {
                            (Up, Up) | (Down, Down) | (Left, Left) | (Right, Right) => {
                                straight_rem - 1
                            }
                            (Up, Down) | (Down, Up) | (Left, Right) | (Right, Left) => 0,
                            (Up, _) | (Down, _) | (Left, _) | (Right, _) => 3,
                        };
                        if new_straight_rem == 0 {
                            None
                        } else {
                            Some((d.0.pos, d.1, new_straight_rem))
                        }
                    } else {
                        None
                    }
                })
                .filter_map(|d| d.map(|d| (d, model.get(d.0.x(), d.0.y()).unwrap())))
                .collect();
            successors
        },
        |(p, _dir, _straight_rem)| p.x().abs_diff(end.x()) + p.y().abs_diff(end.y()),
        |(p, _dir, _straight_rem)| *p == end,
    )
    .unwrap();

    path.1
}

pub fn part2(model: Model) -> Answer {
    use CardDir::*;

    let start: Point<2> = [0, 0].into();
    let end: Point<2> = [model.width() - 1, model.height() - 1].into();

    let path = astar(
        &(start, None, 0),
        |(p, dir, straight)| {
            let successors: Vec<_> = model
                .adj_4(p.x(), p.y())
                .cells
                .into_iter()
                .zip([Up, Left, Right, Down])
                .map(|(new_point, new_dir)| {
                    if let Some(new_point) = new_point {
                        #[rustfmt::skip]
                        let new_straight = match (dir, new_dir) {
                            (None, Down) | (None, Right) | (Some(Up), Up) | (Some(Down), Down) | (Some(Left), Left) | (Some(Right), Right)
                                if *straight < 10 =>
                            {
                                Some(straight + 1)
                            }
                            // no continuing if straight >= 10
                            (Some(Up), Up) | (Some(Down), Down) | (Some(Left), Left) | (Some(Right), Right) => None,
                            // no 180's
                            (Some(Up), Down) | (Some(Down), Up) | (Some(Left), Right) | (Some(Right), Left) => None,
                            // turning allowed if straight >= 4
                            (Some(Up), _) | (Some(Down), _) | (Some(Left), _) | (Some(Right), _)
                                if *straight >= 4 =>
                            {
                                Some(1)
                            }
                            _ => None,
                        };
                        new_straight
                            .map(|new_straight| (new_point.pos, Some(new_dir), new_straight))
                    } else {
                        None
                    }
                })
                .filter_map(|d| d.map(|d| (d, model.get(d.0.x(), d.0.y()).unwrap())))
                .collect();
            successors
        },
        |(p, _dir, _straight_rem)| p.x().abs_diff(end.x()) + p.y().abs_diff(end.y()),
        |(p, _dir, straight_rem)| *p == end && *straight_rem >= 4,
    )
    .expect("couldn't find a path");

    path.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d17");
    const EXAMPLE: &str = include_str!("../examples/d17");
    const EXAMPLE2: &str = include_str!("../examples/d17-2");

    #[test]
    fn d17p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 102);
    }

    #[test]
    fn d17p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 859);
    }

    #[test]
    fn d17p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 94);
    }

    #[test]
    fn d17p2_example2_test() {
        assert_eq!(part2(parse(EXAMPLE2.to_string())), 71);
    }

    #[test]
    fn d17p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 1027);
    }
}
