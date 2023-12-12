//! A solution to day 10 year 2023.
//! https://adventofcode.com/2023/day/10

use std::fmt::Display;

use pathfinding::prelude::bfs_reach;

use crate::{
    grid::{Adj, Cell, Grid},
    point::Point,
};

type Model = Layout;
type Answer = usize;

pub struct Layout {
    grid: Grid<Pipe>,
    start: Point<2>,
}

/// Pipe types.  Directional names like "LeftDown" indicate that the pipe connects to the left, and
/// down.  In pictures, LeftDown is: ╗
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Pipe {
    Start,
    UpDown,
    LeftRight,
    RightDown,
    LeftDown,
    UpRight,
    UpLeft,
    NoPipe,
}

impl Pipe {
    fn from_char(c: char) -> Pipe {
        match c {
            'S' => Pipe::Start,
            '-' => Pipe::LeftRight,
            '|' => Pipe::UpDown,
            'L' => Pipe::UpRight,
            'J' => Pipe::UpLeft,
            'F' => Pipe::RightDown,
            '7' => Pipe::LeftDown,
            _ => Pipe::NoPipe,
        }
    }
    fn to_char(p: &Pipe) -> char {
        match p {
            Pipe::Start => 'S',
            Pipe::UpDown => '║',
            Pipe::LeftRight => '═',
            Pipe::RightDown => '╔',
            Pipe::LeftDown => '╗',
            Pipe::UpRight => '╚',
            Pipe::UpLeft => '╝',
            Pipe::NoPipe => '.',
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Pipe::to_char(self))
    }
}

pub fn parse(input: String) -> Model {
    let mut start = [0, 0];
    let grid = Grid::<Pipe>::new(
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start = [x, y];
                        }
                        Pipe::from_char(c)
                    })
                    .collect()
            })
            .collect(),
    );
    Layout {
        grid,
        start: start.into(),
    }
}

/// Find two pipes connected to the given pipe, and whether the given pipe is a vertex
fn connect(from: Pipe, adj: Adj<Pipe>) -> Option<[Cell<Pipe>; 2]> {
    use Pipe::*;

    let mut a = None;
    let mut b = None;

    enum Dir {
        Up,
        Left,
        Right,
        Down,
    }

    // if `a` is None, assign to `a`, otherwise if `b` is None, assign to `b`, otherwise PANIC
    // because there should be only two connected pipes.
    let mut assign = |cell| match a {
        Some(_) => match b {
            Some(_) => panic!("three connected pipes should be impossible"),
            None => b = Some(cell),
        },
        None => a = Some(cell),
    };

    // U
    if let Some(up) = adj.cells[1] {
        if let (Start | UpDown | UpLeft | UpRight, Start | UpDown | RightDown | LeftDown) =
            (from, up.data)
        {
            assign(up)
        }
    }

    // L
    if let Some(left) = adj.cells[3] {
        if let (Start | LeftRight | LeftDown | UpLeft, Start | LeftRight | RightDown | UpRight) =
            (from, left.data)
        {
            assign(left)
        }
    }

    // R
    if let Some(right) = adj.cells[4] {
        if let (Start | RightDown | LeftRight | UpRight, Start | LeftRight | LeftDown | UpLeft) =
            (from, right.data)
        {
            assign(right)
        }
    }

    // D
    if let Some(down) = adj.cells[6] {
        if let (Start | RightDown | UpDown | LeftDown, Start | UpDown | UpRight | UpLeft) =
            (from, down.data)
        {
            assign(down)
        }
    }

    a.and_then(|a| b.map(|b| [a, b]))
}

pub fn part1(model: Model) -> Answer {
    println!("{}", model.grid);
    println!("start: {}", model.start);

    let start_adj = model.grid.adj(model.start.x(), model.start.y());
    let start_cell = Cell::new(
        model.start,
        model.grid.cells[model.start.y()][model.start.x()],
    );

    // previous point on trail 1
    let mut last1 = start_cell;
    // previous point on trail 2
    let mut last2 = start_cell;

    // get pipes connected to start
    let start_con =
        connect(start_cell.data, start_adj).expect("couldn't find connections to start");
    // current location for trail 1
    let mut loc1 = start_con[0];
    // current location for trail 2
    let mut loc2 = start_con[1];

    let mut steps = 1;

    loop {
        steps += 1;

        // continue finding connections to loc1 and loc2 until they are equal

        let con1 = connect(loc1.data, model.grid.adj(loc1.pos.x(), loc1.pos.y()))
            .unwrap()
            .iter()
            .filter(|&loc| loc != &last1)
            .copied()
            .next()
            .unwrap();

        last1 = loc1;
        loc1 = con1;

        let con2 = connect(loc2.data, model.grid.adj(loc2.pos.x(), loc2.pos.y()))
            .unwrap()
            .iter()
            .filter(|&loc| loc != &last2)
            .copied()
            .next()
            .unwrap();

        last2 = loc2;
        loc2 = con2;

        if loc1 == loc2 {
            break;
        }
    }

    steps
}

pub fn part2(model: Model) -> Answer {
    println!("{}", model.grid);
    println!("start: {}", model.start);

    let start_adj = model.grid.adj(model.start.x(), model.start.y());
    let start_cell = Cell::new(
        model.start,
        model.grid.cells[model.start.y()][model.start.x()],
    );

    // previous point on trail 1
    let mut last1 = start_cell;
    // previous point on trail 2
    let mut last2 = start_cell;

    // get pipes connected to start
    let start_con =
        connect(start_cell.data, start_adj).expect("couldn't find connections to start");
    // current location for trail 1
    let mut loc1 = start_con[0];
    // current location for trail 2
    let mut loc2 = start_con[1];

    let mut steps = 1;

    // the connetged pipes in the loop
    let mut pipes = vec![model.start, loc1.pos, loc2.pos];

    loop {
        steps += 1;

        // continue finding connections to loc1 and loc2 until they are equal

        let con1 = connect(loc1.data, model.grid.adj(loc1.pos.x(), loc1.pos.y()))
            .unwrap()
            .iter()
            .filter(|&loc| loc != &last1)
            .copied()
            .next()
            .unwrap();

        last1 = loc1;
        loc1 = con1;

        let con2 = connect(loc2.data, model.grid.adj(loc2.pos.x(), loc2.pos.y()))
            .unwrap()
            .iter()
            .filter(|&loc| loc != &last2)
            .copied()
            .next()
            .unwrap();

        last2 = loc2;
        loc2 = con2;

        if loc1 == loc2 {
            pipes.push(loc1.pos);
            break;
        } else {
            pipes.push(loc1.pos);
            pipes.push(loc2.pos);
        }
    }

    let outer_locs: Vec<_> = bfs_reach(Point::<2> { coords: [0, 0] }, |&n| {
        model
            .grid
            .adj(n.x(), n.y())
            .cells
            .into_iter()
            .filter_map(|cell| {
                cell.and_then(|c| {
                    print!("can we visit {:?}... ", c.pos);
                    print!("  is in pipe loop? {}...", pipes.contains(&c.pos));
                    let next = (!pipes.contains(&c.pos)).then_some(c.pos);
                    println!("  visit? {:?}", next);
                    next
                })
            })
    })
    .collect();

    let total_area = (model.grid.width() + 1) * (model.grid.height() + 1);
    let total_area = model.grid.area();

    println!("cells outside pipe loop: {}", outer_locs.len());
    println!("total area: {}", total_area);
    println!("pipe loop lenth: {}", pipes.len());
    // dbg!(&pipes);
    for loc in &outer_locs {
        println!("{loc:?}");
    }

    total_area - outer_locs.len() - pipes.len()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const INPUT: &str = include_str!("../input/d10");
//     const EXAMPLE: &str = include_str!("../examples/d10");
//
//     // #[test]
//     // fn d10p1_example_test() {
//     //     assert_eq!(
//     //         part1(parse(EXAMPLE.to_string())),
//     //         "put part 1 example answer here"
//     //     );
//     // }
//
//     // #[test]
//     // fn d10p1_input_test() {
//     //     assert_eq!(
//     //         part1(parse(INPUT.to_string())),
//     //         "put part 1 final answer here"
//     //     );
//     // }
//
//     // #[test]
//     // fn d10p2_example_test() {
//     //     assert_eq!(
//     //         part2(parse(EXAMPLE.to_string())),
//     //         "put part 2 example answer here"
//     //     );
//     // }
//
//     // #[test]
//     // fn d10p2_input_test() {
//     //     assert_eq!(
//     //         part2(parse(INPUT.to_string())),
//     //         "put part 2 final answer here"
//     //     );
//     // }
//
// }
