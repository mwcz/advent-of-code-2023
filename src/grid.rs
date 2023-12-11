use std::fmt::Display;

use crate::point::Point;

pub struct Grid<T: Copy> {
    pub cells: Vec<Vec<T>>,
}

impl<T: Copy> Grid<T> {
    pub fn new(cells: Vec<Vec<T>>) -> Self {
        Self { cells }
    }

    /// Get cells adjacent to the given point.  Origin is top-left.  Cells outside the grid bounds
    /// will be None.
    ///
    /// # Ordering
    ///
    /// Eight cells will always be returned, in the following order relative to the given point:
    ///  
    /// ```
    /// [
    ///   ↖️, ⬆️, ↗️,
    ///   ⬅️,    ➡️,
    ///   ↙️, ⬇️, ↘️,
    /// ]
    /// ```
    ///
    /// In words: up left, up, up right, left, right, down left, down, down right.
    pub fn adj(&self, x: usize, y: usize) -> Adj<T> {
        Adj::new(
            [
                (x.checked_sub(1), y.checked_sub(1)),
                (Some(x), y.checked_sub(1)),
                (x.checked_add(1), y.checked_sub(1)),
                (x.checked_sub(1), Some(y)),
                (x.checked_add(1), Some(y)),
                (x.checked_sub(1), y.checked_add(1)),
                (Some(x), y.checked_add(1)),
                (x.checked_add(1), y.checked_add(1)),
            ]
            .map(|(adj_x, adj_y)| {
                adj_x.and_then(|adj_x| {
                    adj_y.and_then(|adj_y| {
                        self.cells.get(adj_y).and_then(|row| {
                            row.get(adj_x)
                                .map(|cell_data| Cell::new([adj_x, adj_y].into(), *cell_data))
                        })
                    })
                })
            }),
        )
    }
}

impl<T> Display for Grid<T>
where
    T: Display + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{cell}")?;
            }
            println!();
        }
        Ok(())
    }
}

/// A representation of cells adjacent to a point.  Produced by Grid::adj.
#[derive(PartialEq, Debug)]
pub struct Adj<T: Copy> {
    pub cells: [Option<Cell<T>>; 8],
}

/// A cell in a grid, containing some data and a position within the grid.
#[derive(PartialEq, Debug)]
pub struct Cell<T> {
    pub pos: Point<2>,
    pub data: T,
}

impl<T> Cell<T> {
    pub fn new(pos: Point<2>, data: T) -> Self {
        Self { pos, data }
    }
}

impl<T: Copy> Adj<T> {
    pub fn new(cells: [Option<Cell<T>>; 8]) -> Self {
        Self { cells }
    }
    // TODO add up_left(), up(), etc?
}

#[cfg(test)]
mod grid_tests {
    use super::*;

    #[test]
    fn empty_test() {
        let g: Grid<bool> = Grid { cells: vec![] };
        assert_eq!(
            g.adj(0, 0),
            Adj::new([None, None, None, None, None, None, None, None])
        );
        assert_eq!(
            g.adj(1, 1),
            Adj::new([None, None, None, None, None, None, None, None])
        );
    }

    #[test]
    fn one_row_test() {
        let g: Grid<u8> = Grid {
            cells: vec![vec![1, 2, 3, 4, 5, 6, 7]],
        };
        #[rustfmt::skip]
        assert_eq!(
            g.adj(0, 0),
            Adj::new([
                None,                              None,                              None,
                None,                                                                 Some(Cell::new([1,0].into(), 2)),
                None,                              None,                              None,
            ])
        );
        #[rustfmt::skip]
        assert_eq!(
            g.adj(3, 0),
            Adj::new([
                None,                              None,                              None,
                Some(Cell::new([2,0].into(), 3)),                                     Some(Cell::new([4,0].into(), 5)),
                None,                              None,                              None,
            ])
        );
    }

    #[test]
    fn grid_3x3_test() {
        let g: Grid<u8> = Grid {
            #[rustfmt::skip]
            cells:vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9]
            ],
        };
        #[rustfmt::skip]
        assert_eq!(
            g.adj(0, 0),
            Adj::new([
                None,                              None,                              None,
                None,                                                                 Some(Cell::new([1,0].into(), 2)),
                None,                              Some(Cell::new([0, 1].into(), 4)), Some(Cell::new([1,1].into(), 5)),
            ])
        );
        #[rustfmt::skip]
        assert_eq!(
            g.adj(1, 1),
            Adj::new([
                Some(Cell::new([0, 0].into(), 1)), Some(Cell::new([1, 0].into(), 2)), Some(Cell::new([2,0].into(), 3)),
                Some(Cell::new([0, 1].into(), 4)),                                    Some(Cell::new([2,1].into(), 6)),
                Some(Cell::new([0, 2].into(), 7)), Some(Cell::new([1, 2].into(), 8)), Some(Cell::new([2,2].into(), 9)),
            ])
        );
    }
}
