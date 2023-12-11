pub struct Grid<T> {
    pub cells: Vec<Vec<T>>,
}

impl<T> Grid<T> {
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
                    adj_y.and_then(|adj_y| self.cells.get(adj_y).and_then(|row| row.get(adj_x)))
                })
            }),
        )
    }
}

#[derive(PartialEq, Debug)]
pub struct Adj<'a, T> {
    pub cells: [Option<&'a T>; 8],
}

impl<'a, T> Adj<'a, T> {
    pub fn new(cells: [Option<&'a T>; 8]) -> Self {
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
                None, None, None,
                None,       Some(&2),
                None, None, None,
            ])
        );
        assert_eq!(
            g.adj(3, 0),
            Adj::new([None, None, None, Some(&3), Some(&5), None, None, None,])
        );
        assert_eq!(
            g.adj(3, 0),
            Adj::new([None, None, None, Some(&3), Some(&5), None, None, None,])
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
                None,     None,     None,
                None,               Some(&2),
                None,     Some(&4), Some(&5),
            ])
        );
        #[rustfmt::skip]
        assert_eq!(
            g.adj(1, 1),
            Adj::new([
                Some(&1), Some(&2), Some(&3),
                Some(&4),           Some(&6),
                Some(&7), Some(&8), Some(&9),
            ])
        );
    }
}
