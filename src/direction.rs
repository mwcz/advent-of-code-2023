#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CardDir {
    Up,
    Down,
    Left,
    Right,
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
