use std::fmt::Display;

use crate::point::Point;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid<T: Copy> {
    pub cells: Vec<Vec<T>>,
}

impl<T: Copy> Grid<T> {
    pub fn new(cells: Vec<Vec<T>>) -> Self {
        Self { cells }
    }

    pub fn width(&self) -> usize {
        self.cells.first().map(|row| row.len()).unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.cells.len()
    }

    pub fn area(&self) -> usize {
        self.width() * self.height()
    }

    pub fn rows(&self) -> Vec<Vec<T>> {
        self.cells.clone()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        self.cells.get(y).and_then(|row| row.get(x)).copied()
    }

    pub fn cols(&self) -> Vec<Vec<T>> {
        (0..self.cells[0].len())
            .map(|x| (0..self.cells.len()).map(|y| self.cells[y][x]).collect())
            .collect()
    }

    /// Copy the items from new_col into the grid at the given column index (col_idx).
    pub fn set_col(&mut self, col_idx: usize, new_col: &[T]) {
        assert_eq!(new_col.len(), self.height());

        for (i, t) in new_col.iter().enumerate() {
            self.cells[i][col_idx] = *t;
        }
    }

    /// Copy the items from new_row into the grid at the given row index (row_idx).
    pub fn set_row(&mut self, row_idx: usize, new_row: &[T]) {
        assert_eq!(new_row.len(), self.width());

        for (i, t) in new_row.iter().enumerate() {
            self.cells[row_idx][i] = *t;
        }
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
            writeln!(f)?;
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
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
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

    #[test]
    fn grid_set_col_test() {
        let mut g: Grid<u8> = Grid {
            #[rustfmt::skip]
            cells:vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9]
            ],
        };

        g.set_col(1, &[13, 11, 12]);

        #[rustfmt::skip]
        assert_eq!(
            g.cells,
            vec![
                vec![1, 13, 3],
                vec![4, 11, 6],
                vec![7, 12, 9]
            ]
        );
    }

    #[test]
    fn grid_set_row_test() {
        let mut g: Grid<u8> = Grid {
            #[rustfmt::skip]
            cells:vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9]
            ],
        };

        g.set_row(1, &[13, 11, 12]);

        #[rustfmt::skip]
        assert_eq!(
            g.cells,
            vec![
                vec![ 1,  2,  3],
                vec![13, 11, 12],
                vec![ 7,  8,  9]
            ]
        );
    }
}
