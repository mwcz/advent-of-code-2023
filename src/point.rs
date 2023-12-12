use std::fmt::Display;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
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
