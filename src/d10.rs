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

/// Find two pipes connected to the given pipe, and the type of the from pipe (in order to
/// determine the correct starting pipe)
fn connect(from: Pipe, adj: Adj<Pipe>) -> Option<([Cell<Pipe>; 2], Pipe)> {
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

    let mut above = false;
    let mut below = false;
    let mut leftward = false;
    let mut rightward = false;

    // U
    if let Some(up) = adj.cells[1] {
        if let (Start | UpDown | UpLeft | UpRight, Start | UpDown | RightDown | LeftDown) =
            (from, up.data)
        {
            above = true;
            assign(up)
        }
    }

    // L
    if let Some(left) = adj.cells[3] {
        if let (Start | LeftRight | LeftDown | UpLeft, Start | LeftRight | RightDown | UpRight) =
            (from, left.data)
        {
            leftward = true;
            assign(left)
        }
    }

    // R
    if let Some(right) = adj.cells[4] {
        if let (Start | RightDown | LeftRight | UpRight, Start | LeftRight | LeftDown | UpLeft) =
            (from, right.data)
        {
            rightward = true;
            assign(right)
        }
    }

    // D
    if let Some(down) = adj.cells[6] {
        if let (Start | RightDown | UpDown | LeftDown, Start | UpDown | UpRight | UpLeft) =
            (from, down.data)
        {
            below = true;
            assign(down)
        }
    }

    let from_pipe_type = if above && below {
        Pipe::UpDown
    } else if above && leftward {
        Pipe::UpLeft
    } else if above && rightward {
        Pipe::UpRight
    } else if leftward && rightward {
        Pipe::LeftRight
    } else if leftward && below {
        Pipe::LeftDown
    } else if rightward && below {
        Pipe::RightDown
    } else {
        unreachable!();
    };

    a.and_then(|a| b.map(|b| ([a, b], from_pipe_type)))
}

pub fn part1(model: Model) -> Answer {
    // println!("{}", model.grid);
    // println!("start: {}", model.start);

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
    let (start_con, start_type) =
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
            .0
            .iter()
            .filter(|&loc| loc != &last1)
            .copied()
            .next()
            .unwrap();

        last1 = loc1;
        loc1 = con1;

        let con2 = connect(loc2.data, model.grid.adj(loc2.pos.x(), loc2.pos.y()))
            .unwrap()
            .0
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

pub fn part2(mut model: Model) -> Answer {
    // println!("{}", model.grid);
    // println!("start: {}", model.start);

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
    let (start_con, start_type) =
        connect(start_cell.data, start_adj).expect("couldn't find connections to start");
    // current location for trail 1
    let mut loc1 = start_con[0];
    // current location for trail 2
    let mut loc2 = start_con[1];

    // fix start cell
    model.grid.cells[model.start.y()][model.start.x()] = start_type;
    last1.data = start_type;
    last2.data = start_type;
    // println!("{}", model.grid);

    let mut steps = 1;

    // the connected pipes in the loop
    let mut pipes = vec![model.start, loc1.pos, loc2.pos];

    loop {
        steps += 1;

        // continue finding connections to loc1 and loc2 until they are equal

        // find next connection that isn't the previous pipe in trail 1
        let con1 = connect(loc1.data, model.grid.adj(loc1.pos.x(), loc1.pos.y()))
            .unwrap()
            .0
            .iter()
            .filter(|&loc| loc != &last1)
            .copied()
            .next()
            .unwrap();

        last1 = loc1;
        loc1 = con1;

        // find next connection that isn't the previous pipe in trail 1
        let con2 = connect(loc2.data, model.grid.adj(loc2.pos.x(), loc2.pos.y()))
            .unwrap()
            .0
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

    // println!("Pipe loop:");
    // for pipe in &pipes {
    //     println!("  {pipe:?}");
    // }
    //
    // TODO the pipes array is incomplete!

    // it's raycastin' time

    let within = |p: Point<2>| {
        let y = p.y();
        let mut ints = 0;
        // the pipe we're waiting for that indicates entry (╚╗ is in but ╚╝ is out)
        let mut wait_pipe = Pipe::NoPipe;
        let mut inc_next = false;
        for (x, cell) in model.grid.cells[y].iter().enumerate() {
            let in_loop = pipes.contains(&[x, y].into());
            if x == p.x() {
                // hacky short circuit if we end on a loop cell
                if in_loop {
                    return false;
                }
                break;
            }
            if in_loop {
                match cell {
                    Pipe::UpDown => {
                        ints += 1;
                    }
                    Pipe::RightDown => wait_pipe = Pipe::UpLeft,
                    Pipe::UpRight => wait_pipe = Pipe::LeftDown,
                    Pipe::LeftDown if wait_pipe == Pipe::LeftDown => {
                        // "reset" the wait
                        wait_pipe = Pipe::NoPipe;
                        ints += 1;
                    }
                    Pipe::UpLeft if wait_pipe == Pipe::UpLeft => {
                        // "reset" the wait
                        wait_pipe = Pipe::NoPipe;
                        ints += 1;
                    }
                    _ => {}
                }
            } else {
                wait_pipe = Pipe::NoPipe;
            }
        }
        ints % 2 == 1
    };

    let mut count = 0;
    for y in 0..model.grid.height() {
        for x in 0..model.grid.width() {
            let p = [x, y].into();
            let is_in = within(p);
            if is_in {
                // println!("{p:?}");
                count += 1;
            }
        }
    }

    count
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
