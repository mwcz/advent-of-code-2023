//! A solution to day 18 year 2023.
//! https://adventofcode.com/2023/day/18

use crate::{direction::CardDir, grid::Grid, point::Point};

type Answer = String;

#[derive(Debug)]
pub struct Model {
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

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
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
                println!("sum y: {sum_y}");
                min_y = min_y.min(sum_y);
            }
            CardDir::Down => {
                sum_y += step.mag;
                println!("sum y: {sum_y}");
                max_y = max_y.max(sum_y);
            }
            CardDir::Left => {
                sum_x -= step.mag;
                min_x = min_x.min(sum_x);
                println!("sum x: {sum_x}");
            }
            CardDir::Right => {
                sum_x += step.mag;
                println!("sum x: {sum_x}");
                max_x = max_x.max(sum_x);
            }
        }
    }

    let x_offset = -min_x;
    let y_offset = -min_y;
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    Model {
        x_offset,
        y_offset,
        width,
        height,
        grid: Grid::new(vec![vec!['.'; width as usize]; height as usize]),
        steps,
    }
}

pub fn part1(mut model: Model) -> Answer {
    let mut pos: Point<2> = [0, 0].into();
    // print all of model's fields EXCEPT grid
    println!(
        "x_offset: {}, y_offset: {}, width: {}, height: {}",
        model.x_offset, model.y_offset, model.width, model.height
    );
    // println!("steps: {:?}", model.steps);

    for step in &model.steps {
        model.grid.cells[pos.y()][pos.x()] = '#';

        println!("@ {} go {} {} meters", pos, step.dir, step.mag);

        match step.dir {
            CardDir::Up => pos.set_y((model.y_offset + pos.y() as i32 - step.mag) as usize),
            CardDir::Down => pos.set_y((model.y_offset + pos.y() as i32 + step.mag) as usize),
            CardDir::Left => pos.set_x((model.x_offset + pos.x() as i32 - step.mag) as usize),
            CardDir::Right => pos.set_x((model.x_offset + pos.x() as i32 + step.mag) as usize),
        }
    }

    println!("{}", model.grid);
    "incomplete".to_string()
}

pub fn part2(model: Model) -> Answer {
    "incomplete".to_string()
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
