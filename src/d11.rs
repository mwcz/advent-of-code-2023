//! A solution to day 11 year 2023.
//! https://adventofcode.com/2023/day/11

use std::cmp::{self, Ordering};

type Model = (Vec<Vec<u32>>, Vec<usize>, Vec<usize>);
type Answer = usize;

pub fn parse(input: String) -> Model {
    let mut id = 0;

    // 0 is empty
    // 1+ is galaxy

    let mut empty_rows = vec![];
    let mut empty_cols = vec![];

    let mut universe: Vec<Vec<u32>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            // save empty rows
            if line.chars().all(|c| c == '.') {
                empty_rows.push(y);
            }
            line.chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => {
                        id += 1;
                        id
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    // find empty columns

    'outer: for x in 0..universe[0].len() {
        for cell in &universe {
            if cell[x] != 0 {
                continue 'outer;
            }
        }
        empty_cols.push(x);
    }

    (universe, empty_rows, empty_cols)
}

fn uniprint(universe: &[Vec<u32>]) {
    for row in universe.iter() {
        for cell in row.iter() {
            print!(
                "{}",
                if *cell == 0 {
                    '.'
                } else {
                    //
                    '#'
                    // char::from_digit(*cell, 10).unwrap()
                }
            );
        }
        println!();
    }
    println!();
}

pub fn part1((mut universe, empty_rows, empty_cols): Model) -> Answer {
    // embiggen

    for row in universe.iter_mut() {
        for col_idx in empty_cols.iter().rev() {
            row.insert(*col_idx, 0);
        }
    }

    // EMBIGGEN

    let row_len = universe[0].len();
    for row_idx in empty_rows.iter().rev() {
        universe.insert(*row_idx, vec![0; row_len]);
    }

    // uniprint(&universe);

    let mut gals = vec![];
    for (y, row) in universe.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell > 0 {
                gals.push((x, y));
            }
        }
    }

    let mut sum = 0;
    let mut iters = 0;

    for (i, gal_a) in gals.iter().enumerate() {
        for gal_b in gals[(i + 1)..].iter() {
            if gal_a != gal_b {
                iters += 1;
                sum += (gal_a.0.abs_diff(gal_b.0)) + (gal_a.1.abs_diff(gal_b.1));
            }
        }
    }

    sum
}

pub fn part2<const F: usize>((mut universe, empty_rows, empty_cols): Model) -> Answer {
    let mut gals = vec![];
    for (y, row) in universe.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell > 0 {
                gals.push((x, y));
            }
        }
    }

    let mut sum = 0;
    let mut iters = 0;

    for (i, gal_a) in gals.iter().enumerate() {
        for gal_b in gals[(i + 1)..].iter() {
            if gal_a != gal_b {
                iters += 1;

                let y_empty_count = if gal_a.1 == gal_b.1 {
                    // if y values are equal there's no room to add additional expansion
                    0
                } else {
                    // find any extra columns between the two galaxies and mul by 1M
                    empty_rows
                        .iter()
                        .filter(|&&y| {
                            // true if empty_col y value is between gal_a.1 and gal_b.1
                            match gal_a.1.cmp(&gal_b.1) {
                                Ordering::Less => {
                                    let between = gal_a.1 < y && y < gal_b.1;
                                    between
                                }
                                Ordering::Greater => {
                                    let between = gal_b.1 < y && y < gal_a.1;
                                    between
                                }
                                Ordering::Equal => false,
                            }
                        })
                        .count()
                };

                let x_empty_count = if gal_a.0 == gal_b.0 {
                    // if x values are equal there's no room to add additional expansion
                    0
                } else {
                    // find any extra rows between the two galaxies and mul by 1M
                    empty_cols
                        .iter()
                        .filter(|&&y| {
                            // true if empty_col y value is between gal_a.0 and gal_b.0
                            match gal_a.0.cmp(&gal_b.0) {
                                Ordering::Less => {
                                    let between = gal_a.0 < y && y < gal_b.0;
                                    between
                                }

                                Ordering::Greater => {
                                    let between = gal_b.0 < y && y < gal_a.0;
                                    between
                                }
                                Ordering::Equal => false,
                            }
                        })
                        .count()
                };

                let dist = (gal_a.0.abs_diff(gal_b.0)) + (gal_a.1.abs_diff(gal_b.1))
                    - y_empty_count
                    - x_empty_count
                    + y_empty_count * F
                    + x_empty_count * F;
                sum += dist;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d11");
    const EXAMPLE: &str = include_str!("../examples/d11");

    #[test]
    fn d11p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 374);
    }

    #[test]
    fn d11p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 9565386);
    }

    #[test]
    fn d11p2_example10_test() {
        assert_eq!(part2::<10>(parse(EXAMPLE.to_string())), 1030);
    }

    #[test]
    fn d11p2_example100_test() {
        assert_eq!(part2::<100>(parse(EXAMPLE.to_string())), 8410);
    }

    #[test]
    fn d11p2_input_test() {
        assert_eq!(part2::<1_000_000>(parse(INPUT.to_string())), 857986849428);
    }
}
