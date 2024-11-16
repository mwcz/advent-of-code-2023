//! A solution to day 19 year 2023.
//! https://adventofcode.com/2023/day/19

use indexmap::IndexMap;
use rayon::prelude::*;
use std::cmp::{max, min};
use std::ops::Range;
use std::{cmp::Ordering, fmt::Display};

type Model<'a> = (IndexMap<String, Workflow>, Vec<Part>);
type Answer = u64;

pub fn parse(input: String) -> Model<'static> {
    let (workflows, parts) = input.split_at(input.find("\n\n").unwrap());

    let workflows: IndexMap<String, Workflow> = workflows
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

impl Display for PartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PartType::XCool => "x",
                PartType::Musical => "m",
                PartType::Aero => "a",
                PartType::Shiny => "s",
            }
        )
    }
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

#[derive(Debug, Clone)]
pub struct AcceptRange {
    x: Vec<Range<u64>>,
    m: Vec<Range<u64>>,
    a: Vec<Range<u64>>,
    s: Vec<Range<u64>>,
}

#[derive(Debug, Clone)]
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

impl Display for Req {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}:{}",
            self.part_type,
            match self.cmp {
                Ordering::Less => "<",
                Ordering::Equal => "=",
                Ordering::Greater => ">",
            },
            self.mag,
            self.dst
        )
    }
}

impl Display for Workflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{{", self.name)?;
        let reqs = self
            .reqs
            .iter()
            .map(|req| {
                if req.cmp == Ordering::Equal {
                    req.dst.to_string()
                } else {
                    format!("{req}")
                }
            })
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{}", reqs);
        write!(f, "}}")
    }
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

const MAX: u64 = 4000;
pub fn part2((workflows, _): Model) -> u64 {
    let mut sum: u64 = 0;

    let mut part_mags: Vec<Part> = Vec::new();

    for (_, x) in &workflows {
        println!("{x}");
    }

    println!();

    let mut a_paths = Vec::new();

    dfs(
        &workflows,
        workflows.first().unwrap().0,
        AcceptRange {
            x: vec![0..MAX],
            m: vec![0..MAX],
            a: vec![0..MAX],
            s: vec![0..MAX],
        },
        &mut a_paths,
    );

    let a_paths = a_paths.into_iter().map(|r| range_intersect(&r.x));

    println!("{a_paths:#?}");

    todo!("working on it...");
}

fn range_intersect(ranges: &Vec<Range<u64>>) -> Range<u64> {
    let out = ranges
        .clone()
        .into_iter()
        .reduce(|acc, range| {
            let start = max(acc.start, range.start);
            let end = min(acc.end, range.end);
            if start < end {
                start..end
            } else {
                0..0
            }
        })
        .unwrap_or(0..0);
    out
}

/// wfs: the workflows to consider
/// req: the current req being searched
fn dfs(
    wfs: &IndexMap<String, Workflow>,
    wf: &str,
    range: AcceptRange,
    a_paths: &mut Vec<AcceptRange>,
) {
    println!("in workflow {}", wf);
    if let Some(wf) = wfs.get(wf) {
        let mut range = range.clone();
        for req in &wf.reqs {
            // if req.cmp == Ordering::Equal {
            // } else {
            //     match req.part_type {
            //         PartType::XCool => range.x -= req.mag,
            //         PartType::Musical => range.m -= req.mag,
            //         PartType::Aero => range.a -= req.mag,
            //         PartType::Shiny => range.s -= req.mag,
            //     }
            // }

            println!("  req {}", req);
            // print if final, otherwise add to search
            if req.dst == "A" {
                // if Accept was reached by the last rule, no range change
                if req.cmp == Ordering::Equal {
                    println!("  complete: {range:?}");
                    a_paths.push(range.clone());
                } else {
                    match (req.part_type, req.cmp) {
                        (PartType::XCool, Ordering::Less) => range.x.push(0..req.mag),
                        (PartType::XCool, Ordering::Greater) => range.x.push(req.mag..MAX),
                        (PartType::Musical, Ordering::Less) => range.m.push(0..req.mag),
                        (PartType::Musical, Ordering::Greater) => range.m.push(req.mag..MAX),
                        (PartType::Aero, Ordering::Less) => range.a.push(0..req.mag),
                        (PartType::Aero, Ordering::Greater) => range.a.push(req.mag..MAX),
                        (PartType::Shiny, Ordering::Less) => range.s.push(0..req.mag),
                        (PartType::Shiny, Ordering::Greater) => range.s.push(req.mag..MAX),
                        (_, _) => {}
                    }
                    // match req.part_type {
                    //     PartType::XCool => range.x -= req.mag,
                    //     PartType::Musical => range.m -= req.mag,
                    //     PartType::Aero => range.a -= req.mag,
                    //     PartType::Shiny => range.s -= req.mag,
                    // }
                    // println!("  complete: {range:?}");
                    println!("  complete: {range:?}");
                    a_paths.push(range.clone());
                }
            } else {
                // not yet complete, keep searching
                dfs(wfs, &req.dst, range.clone(), a_paths);
            }
        }
    }
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

    #[test]
    fn d19p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 167409079868000);
    }

    // #[test]
    // fn d19p2_input_test() {
    //     assert_eq!(
    //         part2(parse(INPUT.to_string())),
    //         "put part 2 final answer here"
    //     );
    // }
}
