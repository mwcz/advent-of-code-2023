//! A solution to day 18 year 2023.
//! https://adventofcode.com/2023/day/18

use crate::direction::CardDir;

// plan for part 1 and part 2
type Model = (Plan, Plan);
type Answer = i64;

#[derive(Debug)]
pub struct Plan {
    /// how far to shift the grid to the right to account for the instructions drifting into
    /// negative x
    x_offset: i64,
    /// how far to shift the grid down to account for the instructions drifting into
    /// negative y
    y_offset: i64,
    steps: Vec<Step>,
}

#[derive(Debug)]
struct Step {
    dir: CardDir,
    mag: i64,
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
    fn get_offsets(steps: &[Step]) -> (i64, i64) {
        let mut sum_x = 0;
        let mut min_x = 0;
        let mut max_x = 0;

        let mut sum_y = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for step in steps.iter() {
            match step.dir {
                CardDir::Up => {
                    sum_y -= step.mag;
                    min_y = min_y.min(sum_y);
                }
                CardDir::Down => {
                    sum_y += step.mag;
                    max_y = max_y.max(sum_y);
                }
                CardDir::Left => {
                    sum_x -= step.mag;
                    min_x = min_x.min(sum_x);
                }
                CardDir::Right => {
                    sum_x += step.mag;
                    max_x = max_x.max(sum_x);
                }
            }
        }

        let x_offset = -min_x + 1;
        let y_offset = -min_y + 1;

        (x_offset, y_offset)
    }

    let p1_plan = {
        let steps: Vec<Step> = input.lines().map(Step::from).collect();

        let (x_offset, y_offset) = get_offsets(&steps);

        Plan {
            x_offset,
            y_offset,
            steps,
        }
    };

    let p2_plan = {
        let steps: Vec<Step> = input
            .lines()
            .map(|line| {
                let (_, hex) = line.split_at(1 + line.find('#').unwrap());
                let dist_s = &hex[0..5];
                let dist = i64::from_str_radix(dist_s, 16).unwrap();
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

        let (x_offset, y_offset) = get_offsets(&steps);

        Plan {
            x_offset,
            y_offset,
            steps,
        }
    };

    (p1_plan, p2_plan)
}

fn solve(plan: Plan) -> Answer {
    let mut x = plan.x_offset;
    let mut y = plan.y_offset;
    let mut lengths = 0;

    let verts: Vec<_> = plan
        .steps
        .iter()
        .map(|step| {
            lengths += step.mag - 1;
            match step.dir {
                CardDir::Up => y -= step.mag,
                CardDir::Down => y += step.mag,
                CardDir::Left => x -= step.mag,
                CardDir::Right => x += step.mag,
            };
            //
            (x, y)
        })
        .collect();

    let mut a = 0;
    for i in 0..verts.len() {
        let j = (i + 1) % verts.len();
        let s1 = verts[i];
        let s2 = verts[j];
        a += s1.0 * s2.1;
        a -= s2.0 * s1.1;
    }
    a = a.abs() / 2;

    // find extra area

    let corners = plan.steps.len() as i64;
    let outies = (corners + 4) / 2;
    let innies = outies - 4;
    let corner_area = (outies * 3 + innies) / 4;
    let length_area = lengths / 2;

    a + corner_area + length_area
}

pub fn part1((model, _): Model) -> Answer {
    solve(model)
}

pub fn part2((_, model): Model) -> Answer {
    solve(model)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d18");
    const EXAMPLE: &str = include_str!("../examples/d18");

    #[test]
    fn d18p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 62);
    }

    #[test]
    fn d18p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 40131);
    }

    #[test]
    fn d18p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 952408144115);
    }

    #[test]
    fn d18p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 104454050898331);
    }
}
