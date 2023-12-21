//! A solution to day 16 year 2023.
//! https://adventofcode.com/2023/day/16

use crate::{direction::CardDir, grid::Grid, point::Point};
use console_engine::{pixel, Color, ConsoleEngine, KeyCode};
use std::{collections::HashSet, fmt::Display};

type Model = Grid<Tile>;
type Answer = usize;

pub fn parse(input: String) -> Model {
    Grid::new(
        input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect(),
    )
}

fn solve(initial_beam: Beam, model: &Model) -> Answer {
    let mut beams = vec![initial_beam];
    let mut energized: HashSet<Beam> = HashSet::new();

    #[cfg(feature = "visualize")]
    let mut engine = ConsoleEngine::init(model.width() as u32, model.height() as u32, 144)
        .expect("console visualization couldn't start");

    use CardDir::*;
    use Mirror::*;
    use Splitter::*;

    while let Some(beam) = beams.pop() {
        energized.insert(beam);

        #[cfg(feature = "visualize")]
        {
            if engine.is_key_pressed(KeyCode::Char('q')) || engine.is_key_held(KeyCode::Char('q')) {
                std::process::exit(1);
            }
            // print map
            engine.wait_frame();
            engine.clear_screen();
            let energized_points: Vec<_> = energized.iter().map(|beam| beam.pos).collect();
            for y in 0..model.height() {
                for x in 0..model.width() {
                    if beam.pos.x() == x && beam.pos.y() == y {
                        engine.set_pxl(x as i32, y as i32, pixel::pxl_bg('@', Color::Cyan));
                    } else if let Some(tile) = model.get(x, y) {
                        if tile == Tile::Empty && energized_points.contains(&[x, y].into()) {
                            engine.set_pxl(x as i32, y as i32, pixel::pxl_bg(' ', Color::DarkRed));
                        } else {
                            match tile {
                                Tile::Empty => {
                                    engine.set_pxl(
                                        x as i32,
                                        y as i32,
                                        pixel::pxl_bg(' ', Color::Black),
                                    );
                                }
                                Tile::Mirror(mirror)
                                    if energized_points.contains(&[x, y].into()) =>
                                {
                                    engine.set_pxl(
                                        x as i32,
                                        y as i32,
                                        pixel::pxl_bg(mirror.into(), Color::DarkRed),
                                    );
                                }
                                Tile::Mirror(mirror) => {
                                    engine.set_pxl(
                                        x as i32,
                                        y as i32,
                                        pixel::pxl_bg(mirror.into(), Color::Black),
                                    );
                                }
                                Tile::Splitter(split)
                                    if energized_points.contains(&[x, y].into()) =>
                                {
                                    engine.set_pxl(
                                        x as i32,
                                        y as i32,
                                        pixel::pxl_bg(split.into(), Color::DarkRed),
                                    );
                                }
                                Tile::Splitter(split) => {
                                    engine.set_pxl(
                                        x as i32,
                                        y as i32,
                                        pixel::pxl_bg(split.into(), Color::Black),
                                    );
                                }
                            }
                        }
                    } else if energized_points.contains(&[x, y].into()) {
                        engine.print(x as i32, y as i32, "#");
                    } else {
                        engine.set_pxl(x as i32, y as i32, pixel::pxl_bg(' ', Color::Black));
                    }
                }
            }
            engine.draw();
        }

        let new_beams = match (beam.dir, model.get(beam.pos.x(), beam.pos.y())) {
            // splitters
            (Up | Down, Some(Tile::Splitter(Dash))) => {
                Some(vec![beam.with_dir(Left), beam.with_dir(Right)])
            }
            (Left | Right, Some(Tile::Splitter(Pipe))) => {
                Some(vec![beam.with_dir(Up), beam.with_dir(Down)])
            }

            // mirrors /
            (Left, Some(Tile::Mirror(Slash))) => Some(vec![beam.with_dir(Down)]),
            (Right, Some(Tile::Mirror(Slash))) => Some(vec![beam.with_dir(Up)]),
            (Up, Some(Tile::Mirror(Slash))) => Some(vec![beam.with_dir(Right)]),
            (Down, Some(Tile::Mirror(Slash))) => Some(vec![beam.with_dir(Left)]),

            // mirrors \
            (Left, Some(Tile::Mirror(Slosh))) => Some(vec![beam.with_dir(Up)]),
            (Right, Some(Tile::Mirror(Slosh))) => Some(vec![beam.with_dir(Down)]),
            (Up, Some(Tile::Mirror(Slosh))) => Some(vec![beam.with_dir(Left)]),
            (Down, Some(Tile::Mirror(Slosh))) => Some(vec![beam.with_dir(Right)]),

            // proceed uninterrupted
            (_, Some(Tile::Empty | Tile::Splitter(_))) => Some(vec![beam]),

            // moved out of bounds
            (_, None) => None,
        };

        if let Some(new_beams) = new_beams {
            for new_beam in new_beams {
                if let Some(new_pos) = new_beam.pos.move_in_grid(new_beam.dir, model) {
                    // continue beam only if this tile hasn't been energized already by a beam traveling in
                    // the same direction (prevent infinite loop)
                    let new_beam = new_beam.with_pos(new_pos);
                    if !energized.contains(&new_beam) {
                        beams.push(new_beam);
                    }
                }
            }
        }
    }

    let mut energized_points: Vec<_> = energized.iter().map(|beam| beam.pos).collect();
    energized_points.sort();
    energized_points.dedup();
    energized_points.len()
}

pub fn part1(model: Model) -> Answer {
    solve(
        Beam {
            dir: CardDir::Right,
            pos: [0, 0].into(),
        },
        &model,
    )
}

pub fn part2(model: Model) -> Answer {
    let mut max = 0;
    for y in 0..model.height() {
        max = max.max(solve(
            Beam {
                dir: CardDir::Right,
                pos: [0, y].into(),
            },
            &model,
        ));
        max = max.max(solve(
            Beam {
                dir: CardDir::Left,
                pos: [model.width() - 1, y].into(),
            },
            &model,
        ));
    }
    for x in 0..model.width() {
        max = max.max(solve(
            Beam {
                dir: CardDir::Down,
                pos: [x, 0].into(),
            },
            &model,
        ));
        max = max.max(solve(
            Beam {
                dir: CardDir::Up,
                pos: [x, model.height() - 1].into(),
            },
            &model,
        ));
    }
    max
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Beam {
    pos: Point<2>,
    dir: CardDir,
}

impl Beam {
    fn with_dir(&self, dir: CardDir) -> Beam {
        let mut b = *self;
        b.dir = dir;
        b
    }
    fn with_pos(&self, pos: Point<2>) -> Beam {
        let mut b = *self;
        b.pos = pos;
        b
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tile {
    Empty,
    Mirror(Mirror),
    Splitter(Splitter),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '-' | '|' => Self::Splitter(Splitter::from(value)),
            '\\' | '/' => Self::Mirror(Mirror::from(value)),
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Splitter(splitter) => write!(f, "{splitter}")?,
            Tile::Mirror(mirror) => write!(f, "{mirror}")?,
            Tile::Empty => write!(f, ".")?,
            _ => unreachable!(),
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mirror {
    Slash,
    Slosh,
}

impl From<char> for Mirror {
    fn from(value: char) -> Self {
        match value {
            '\\' => Self::Slosh,
            '/' => Self::Slash,
            _ => unreachable!(),
        }
    }
}

impl From<Mirror> for char {
    fn from(value: Mirror) -> Self {
        match value {
            Mirror::Slosh => '╲',
            Mirror::Slash => '╱',
            _ => unreachable!(),
        }
    }
}

impl Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mirror::Slosh => '╲',
                Mirror::Slash => '╱',
                _ => unreachable!(),
            }
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Splitter {
    Pipe,
    Dash,
}

impl From<char> for Splitter {
    fn from(value: char) -> Self {
        match value {
            '-' => Self::Dash,
            '|' => Self::Pipe,
            _ => unreachable!(),
        }
    }
}

impl From<Splitter> for char {
    fn from(value: Splitter) -> Self {
        match value {
            Splitter::Dash => '━',
            Splitter::Pipe => '┃',
            _ => unreachable!(),
        }
    }
}

impl Display for Splitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Splitter::Dash => '-',
                Splitter::Pipe => '|',
                _ => unreachable!(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d16");
    const EXAMPLE: &str = include_str!("../examples/d16");

    #[test]
    fn d16p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 46,);
    }

    #[test]
    fn d16p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 7472);
    }

    #[test]
    fn d16p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 51);
    }

    #[test]
    fn d16p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 7716);
    }
}
