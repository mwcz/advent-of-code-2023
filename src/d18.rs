//! A solution to day 18 year 2023.
//! https://adventofcode.com/2023/day/18

use std::collections::HashSet;

use crate::{direction::CardDir, grid::Grid, point::Point};

// plan for part 1 and part 2
type Model = (Plan, Plan);
type Answer = usize;

#[derive(Debug)]
pub struct Plan {
    /// how far to shift the grid to the right to account for the instructions drifting into
    /// negative x
    x_offset: i32,
    /// how far to shift the grid down to account for the instructions drifting into
    /// negative y
    y_offset: i32,
    width: i32,
    height: i32,
    grid: Grid<char>,
    steps: Vec<Step>,
}

#[derive(Debug)]
struct Step {
    dir: CardDir,
    mag: i32,
    // TODO add color
}

impl<S: AsRef<str>> From<S> for Step {
    fn from(value: S) -> Self {
        let mut parts = value.as_ref().split_whitespace();
        let dir_text = parts.next().unwrap();

        let dir = match dir_text {
            "U" => CardDir::Up,
            "L" => CardDir::Left,
            "R" => CardDir::Right,
            "D" => CardDir::Down,
            _ => unreachable!(),
        };

        let mag_text = parts.next().unwrap();

        let mag = mag_text.parse().unwrap();

        Step { dir, mag }
    }
}

pub fn parse(input: String) -> Model {
    let p1_plan = {
        let steps: Vec<Step> = input.lines().map(Step::from).collect();

        let mut sum_x = 0;
        let mut min_x = 0;
        let mut max_x = 0;

        let mut sum_y = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for step in &steps {
            match step.dir {
                CardDir::Up => {
                    sum_y -= step.mag;
                    // println!("sum y: {sum_y}");
                    min_y = min_y.min(sum_y);
                }
                CardDir::Down => {
                    sum_y += step.mag;
                    // println!("sum y: {sum_y}");
                    max_y = max_y.max(sum_y);
                }
                CardDir::Left => {
                    sum_x -= step.mag;
                    min_x = min_x.min(sum_x);
                    // println!("sum x: {sum_x}");
                }
                CardDir::Right => {
                    sum_x += step.mag;
                    // println!("sum x: {sum_x}");
                    max_x = max_x.max(sum_x);
                }
            }
        }

        let x_offset = -min_x + 1;
        let y_offset = -min_y + 1;
        let width = max_x - min_x + 2 + x_offset;
        let height = max_y - min_y + 2 + y_offset;

        Plan {
            x_offset,
            y_offset,
            width,
            height,
            grid: Grid::new(vec![vec!['.'; (width) as usize]; (height) as usize]),
            steps,
        }
    };

    let p2_plan = {
        let steps: Vec<Step> = input
            .lines()
            .map(|line| {
                let (_, hex) = line.split_at(1 + line.find('#').unwrap());
                println!("{hex}");
                let dist_s = &hex[0..5];
                let dist = i32::from_str_radix(dist_s, 16).unwrap();
                let dir_s = &hex[5..6];
                let dir = match dir_s {
                    "0" => "R",
                    "1" => "D",
                    "2" => "L",
                    "3" => "U",
                    _ => unreachable!(),
                };
                format!("{} {}", dir, dist)
            })
            .map(Step::from)
            .collect();

        let mut sum_x = 0;
        let mut min_x = 0;
        let mut max_x = 0;

        let mut sum_y = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for step in &steps {
            match step.dir {
                CardDir::Up => {
                    sum_y -= step.mag;
                    // println!("sum y: {sum_y}");
                    min_y = min_y.min(sum_y);
                }
                CardDir::Down => {
                    sum_y += step.mag;
                    // println!("sum y: {sum_y}");
                    max_y = max_y.max(sum_y);
                }
                CardDir::Left => {
                    sum_x -= step.mag;
                    min_x = min_x.min(sum_x);
                    // println!("sum x: {sum_x}");
                }
                CardDir::Right => {
                    sum_x += step.mag;
                    // println!("sum x: {sum_x}");
                    max_x = max_x.max(sum_x);
                }
            }
        }

        let x_offset = -min_x + 1;
        let y_offset = -min_y + 1;
        let width = max_x - min_x + 2 + x_offset;
        let height = max_y - min_y + 2 + y_offset;

        Plan {
            x_offset,
            y_offset,
            width,
            height,
            grid: Grid::new(vec![vec!['.'; (width) as usize]; (height) as usize]),
            steps,
        }
    };

    (p1_plan, p2_plan)
}

fn solve(mut plan: Plan) -> Answer {
    let mut pos: Point<2> = [plan.x_offset as usize, plan.y_offset as usize].into();
    // print all of model's fields EXCEPT grid
    println!(
        "x_offset: {}, y_offset: {}, width: {}, height: {}",
        plan.x_offset, plan.y_offset, plan.width, plan.height
    );
    // println!("steps: {:?}", model.steps);

    let mut set_tile = |x: usize, y: usize, c: char| {
        println!("O set tile ({x}, {y})");
        plan.grid.cells[y][x] = c;
    };

    for step in &plan.steps {
        println!("@ {} go {} {} meters", pos, step.dir, step.mag);
        let x = (pos.x() as i32);
        let y = (pos.y() as i32);

        let range = match step.dir {
            CardDir::Up => (y - step.mag)..=y,
            CardDir::Down => y..=(y + step.mag),
            CardDir::Left => (x - step.mag)..=x,
            CardDir::Right => x..=(x + step.mag),
        };

        match step.dir {
            CardDir::Up | CardDir::Down => {
                // process vert range
                for y in range.clone() {
                    set_tile(pos.x(), y as usize, '#');
                }
            }
            CardDir::Left | CardDir::Right => {
                // process horiz range
                for x in range.clone() {
                    set_tile(x as usize, pos.y(), '#');
                }
            }
        }
        println!("{range:?}");
        // update current pos
        match step.dir {
            CardDir::Up => pos.set_y(range.min().unwrap() as usize),
            CardDir::Down => pos.set_y(range.max().unwrap() as usize),
            CardDir::Left => pos.set_x(range.min().unwrap() as usize),
            CardDir::Right => pos.set_x(range.max().unwrap() as usize),
        }
    }

    // calculate area with flood fill
    let mut visited: HashSet<Point<2>> = HashSet::new();
    let mut queue: Vec<Point<2>> = vec![[0, 0].into()];

    while let Some(pos) = queue.pop() {
        if !visited.contains(&pos) {
            visited.insert(pos);
            let adj = plan.grid.adj_4(pos.x(), pos.y());

            for cell in adj.cells.into_iter().flatten() {
                if cell.data == '.' && !visited.contains(&cell.pos) {
                    queue.push(cell.pos);
                }
            }
        }
    }

    plan.grid.width() * plan.grid.height() - visited.len()
}

pub fn part1((mut model, _): Model) -> Answer {
    solve(model)
}

pub fn part2((_, mut model): Model) -> Answer {
    solve(model)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const INPUT: &str = include_str!("../input/d18");
//     const EXAMPLE: &str = include_str!("../examples/d18");
//
//     // #[test]
//     // fn d18p1_example_test() {
//     //     assert_eq!(
//     //         part1(parse(EXAMPLE.to_string())),
//     //         "put part 1 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d18p1_input_test() {
//     //     assert_eq!(
//     //         part1(parse(INPUT.to_string())),
//     //         "put part 1 final answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d18p2_example_test() {
//     //     assert_eq!(
//     //         part2(parse(EXAMPLE.to_string())),
//     //         "put part 2 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d18p2_input_test() {
//     //     assert_eq!(
//     //         part2(parse(INPUT.to_string())),
//     //         "put part 2 final answer here"
//     //     );
//     // }
// }
