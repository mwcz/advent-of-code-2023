use std::fmt::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CardDir {
    Up,
    Down,
    Left,
    Right,
}

impl Display for CardDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CardDir::Up => "⬆️".to_string(),
                CardDir::Down => "⬇️".to_string(),
                CardDir::Left => "⬅️".to_string(),
                CardDir::Right => "➡️".to_string(),
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrdDir {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}
