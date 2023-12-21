use std::fmt::Display;

use crate::{direction::CardDir, grid::Grid};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Ord, PartialOrd)]
pub struct Point<const D: usize> {
    pub coords: [usize; D],
}

impl<const D: usize> Point<D> {
    pub fn new(coords: &[usize; D]) -> Self {
        Self { coords: *coords }
    }

    pub fn x(&self) -> usize {
        self.coords[0]
    }
    pub fn y(&self) -> usize {
        self.coords[1]
    }
    pub fn z(&self) -> usize {
        self.coords[2]
    }

    /// Set a new value for the x coordinate.
    pub fn set_x(&mut self, new_x: usize) {
        self.coords[0] = new_x;
    }

    /// Set a new value for the y coordinate.
    pub fn set_y(&mut self, new_y: usize) {
        self.coords[1] = new_y;
    }

    /// Set a new value for the z coordinate.
    pub fn set_z(&mut self, new_z: usize) {
        self.coords[2] = new_z;
    }

    /// Attempt to move the point one unit in the given direction, within a grid bounds.  Returns
    /// None if the move would push the point outside the bounds of the grid.
    pub fn move_in_grid<T: Copy>(&self, dir: CardDir, grid: &Grid<T>) -> Option<Point<D>> {
        let mut p = *self;

        match dir {
            CardDir::Up => p.set_y(p.y().checked_sub(1)?),
            CardDir::Down => p.set_y(p.y().checked_add(1)?),
            CardDir::Left => p.set_x(p.x().checked_sub(1)?),
            CardDir::Right => p.set_x(p.x().checked_add(1)?),
        }

        if grid.width() > p.x() && grid.height() > p.y() {
            Some(p)
        } else {
            None
        }
    }
}

// Make possible the nice pattern `&[1,2,3].into()` to create a Point.
impl<const D: usize> From<&[usize; D]> for Point<D> {
    fn from(coords: &[usize; D]) -> Self {
        Point { coords: *coords }
    }
}

// Make possible the nice pattern `[1,2,3].into()` to create a Point.
impl<const D: usize> From<[usize; D]> for Point<D> {
    fn from(coords: [usize; D]) -> Self {
        Point { coords }
    }
}

impl<const D: usize> Display for Point<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, n) in self.coords.iter().enumerate() {
            write!(f, "{}", n)?;
            if i < self.coords.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}
