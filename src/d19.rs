//! A solution to day 19 year 2023.
//! https://adventofcode.com/2023/day/19

use rayon::prelude::*;
use std::{cmp::Ordering, collections::HashMap};

type Model<'a> = (HashMap<String, Workflow>, Vec<Part>);
type Answer = u64;

pub fn parse(input: String) -> Model<'static> {
    let (workflows, parts) = input.split_at(input.find("\n\n").unwrap());

    let workflows: HashMap<String, Workflow> = workflows
        .trim()
        .lines()
        .map(|line| {
            let brace = line.find('{').unwrap();
            let name = &line[0..brace];
            let reqs = line[brace + 1..line.len() - 1]
                .split(',')
                .map(|req| {
                    let (cmp, idx) = if req.contains('<') {
                        (Ordering::Less, req.find('<').unwrap())
                    } else if req.contains('>') {
                        (Ordering::Greater, req.find('>').unwrap())
                    } else {
                        // Equal implies final
                        (Ordering::Equal, 0)
                    };

                    let colon = req.find(':');

                    let mut part_type;
                    let mut mag;
                    let mut dst;
                    if let Some(colon_idx) = colon {
                        let (pt, mg, dt) = if idx != 0 {
                            (
                                PartType::from(&req[0..idx]),
                                req[idx + 1..colon_idx].parse().unwrap(),
                                &req[colon_idx + 1..],
                            )
                        } else {
                            unreachable!();
                        };
                        part_type = pt;
                        mag = mg;
                        dst = dt;
                    } else {
                        part_type = PartType::XCool;
                        mag = 0;
                        dst = req;
                    }

                    Req {
                        part_type,
                        cmp,
                        mag,
                        dst: dst.to_string(),
                    }
                })
                .collect();

            (
                name.to_string(),
                Workflow {
                    name: name.to_string(),
                    reqs,
                },
            )
        })
        .collect();

    let mut part_queue: Vec<Part> = parts
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();
            let line = &line[1..(line.len() - 1)];
            let mut ratings = line.split(',').map(|r| &r[2..]);
            Part {
                wf: "in".to_string(),
                x: ratings.next().unwrap().parse().unwrap(),
                m: ratings.next().unwrap().parse().unwrap(),
                a: ratings.next().unwrap().parse().unwrap(),
                s: ratings.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    (workflows, part_queue)
}

#[derive(Debug, Copy, Clone)]
pub enum PartType {
    XCool,
    Musical,
    Aero,
    Shiny,
}

impl From<&str> for PartType {
    fn from(value: &str) -> Self {
        match value {
            "x" => Self::XCool,
            "m" => Self::Musical,
            "a" => Self::Aero,
            "s" => Self::Shiny,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Part {
    wf: String,
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn get_type(&self, part_type: PartType) -> u64 {
        match part_type {
            PartType::XCool => self.x,
            PartType::Musical => self.m,
            PartType::Aero => self.a,
            PartType::Shiny => self.s,
        }
    }

    fn with_wf(&self, dst: &str) -> Part {
        Part {
            wf: dst.to_string(),
            x: self.x,
            m: self.m,
            a: self.a,
            s: self.s,
        }
    }
}

#[derive(Debug)]
pub struct Workflow {
    name: String,
    reqs: Vec<Req>,
}

#[derive(Debug)]
pub struct Req {
    part_type: PartType,
    cmp: Ordering,
    mag: u64,
    dst: String,
}

pub fn part1((workflows, mut part_queue): Model) -> Answer {
    let mut sum = 0;
    let mut accepted: Vec<Part> = vec![];

    while let Some(part) = part_queue.pop() {
        let wf = workflows.get(part.wf.as_str()).unwrap();

        // default to the last item
        let mut dst = &wf.reqs.last().unwrap().dst;

        for req in &wf.reqs {
            if part.get_type(req.part_type).cmp(&req.mag) == req.cmp {
                dst = &req.dst;
                break;
                //
            }
        }
        if dst == "A" {
            accepted.push(part);
        } else if dst != "R" {
            part_queue.push(part.with_wf(dst));
        }
    }

    accepted
        .iter()
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

pub fn part2((workflows, _): Model) -> u64 {
    let mut sum: u64 = 0;

    const N: u64 = 4000;

    let range = 0..N.pow(4);

    range
        .into_par_iter()
        .map(|i| {
            let mut part_queue = vec![];
            let x = 1 + i % N;
            let m = 1 + (i / N) % N;
            let a = 1 + (i / N.pow(2)) % N;
            let s = 1 + (i / N.pow(3)) % N;
            let part = Part {
                wf: "in".to_string(),
                x,
                m,
                a,
                s,
            };
            part_queue.push(part);

            while let Some(part) = part_queue.pop() {
                // println!("{:?}", part);
                let wf = workflows.get(part.wf.as_str()).unwrap();

                // default to the last item
                let mut dst = &wf.reqs.last().unwrap().dst;

                for req in &wf.reqs {
                    if part.get_type(req.part_type).cmp(&req.mag) == req.cmp {
                        dst = &req.dst;
                        break;
                        //
                    }
                }
                if dst == "A" {
                    return part.x + part.m + part.a + part.s;
                } else if dst != "R" {
                    part_queue.push(part.with_wf(dst));
                } else {
                    return 0;
                }
            }

            unreachable!();
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d19");
    const EXAMPLE: &str = include_str!("../examples/d19");

    #[test]
    fn d19p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 19114);
    }

    #[test]
    fn d19p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 492702);
    }

    // #[test]
    // fn d19p2_example_test() {
    //     assert_eq!(
    //         part2(parse(EXAMPLE.to_string())),
    //         "put part 2 example answer here"
    //     );
    // }
    //
    // #[test]
    // fn d19p2_input_test() {
    //     assert_eq!(
    //         part2(parse(INPUT.to_string())),
    //         "put part 2 final answer here"
    //     );
    // }
}
