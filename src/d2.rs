//! A solution to day 2 year 2023.
//! https://adventofcode.com/2023/day/2

type Model = Vec<Game>;
type Answer = u32;

pub fn parse(input: String) -> Model {
    input.lines().map(Game::from).collect()
}

pub fn part1(model: Model) -> Answer {
    let bag = Color {
        r: 12,
        g: 13,
        b: 14,
    };
    model.iter().filter_map(|game| game.is_possible(&bag)).sum()
}

pub fn part2(model: Model) -> Answer {
    model.iter().map(|g| g.min_set().power()).sum()
}

#[derive(Debug)]
struct Color {
    r: u32,
    g: u32,
    b: u32,
}

impl Color {
    fn fits_in(&self, other: &Self) -> bool {
        self.r <= other.r && self.g <= other.g && self.b <= other.b
    }

    fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        let mut color = Color { r: 0, g: 0, b: 0 };
        value.trim().split(',').for_each(|c| {
            let (num, name) = c.trim().split_once(' ').unwrap();
            let num = num.parse().unwrap();
            match name {
                "red" => color.r = num,
                "green" => color.g = num,
                "blue" => color.b = num,
                _ => unreachable!(),
            }
        });
        color
    }
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    colors: Vec<Color>,
}

impl Game {
    /// Return Some(id) if the game is possible, otherwise None
    fn is_possible(&self, bag: &Color) -> Option<u32> {
        self.colors
            .iter()
            .all(|color| color.fits_in(bag))
            .then_some(self.id)
    }

    fn min_set(&self) -> Color {
        self.colors
            .iter()
            .fold(Color { r: 0, g: 0, b: 0 }, |a, b| Color {
                r: a.r.max(b.r),
                g: a.g.max(b.g),
                b: a.b.max(b.b),
            })
    }
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let (game_part, color_part) = line.split_once(':').unwrap();
        let id: u32 = game_part.replace("Game ", "").parse().unwrap();
        let colors = color_part.split(';').map(Color::from).collect();
        Game { id, colors }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d2");
    const EXAMPLE: &str = include_str!("../examples/d2");

    #[test]
    fn d2p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 8);
    }

    #[test]
    fn d2p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 1867);
    }

    #[test]
    fn d2p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 2286);
    }

    #[test]
    fn d2p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 84538);
    }
}
