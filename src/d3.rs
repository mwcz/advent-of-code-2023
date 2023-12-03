//! A solution to day 3 year 2023.
//! https://adventofcode.com/2023/day/3

type Model = Vec<Vec<char>>;
type Answer = u32;

pub fn parse(input: String) -> Model {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part1(model: Model) -> Answer {
    let ymax = model.len() - 1;
    let xmax = model[0].len() - 1;

    fn is_symbol(c: char) -> bool {
        !c.is_ascii_digit() && c != '.'
    }

    let is_symbol_adjacent = |x: usize, y: usize| {
        if y == 137 && x == 65 {
            println!("STOP!");
        }
        let y_vals = [
            y.checked_sub(1),
            Some(y),
            ((y + 1) <= ymax).then_some(y + 1),
        ]
        .into_iter()
        .flatten();

        let x_vals = [
            x.checked_sub(1),
            Some(x),
            ((x + 1) <= xmax).then_some(x + 1),
        ]
        .into_iter()
        .flatten();

        for y_adj in y_vals {
            for x_adj in x_vals.clone() {
                if is_symbol(model[y_adj][x_adj]) {
                    return true;
                }
            }
        }

        false

        // model[y - 1][x - 1];
        // model[y - 1][x + 0];
        // model[y - 1][x + 1];
        // model[y + 0][x - 1];
        // model[y + 0][x + 1];
        // model[y + 1][x - 1];
        // model[y + 1][x + 0];
        // model[y + 1][x + 1];
        //
        // if y > 0 {
        //     let tl = model[y - 1][x - 1];
        //     let tm = model[y - 1][x + 0];
        //     let tr = model[y - 1][x + 1];
        // }
        // let l = model[y + 0][x - 1];
        // let r = model[y + 0][x + 1];
        // if y < ymax {
        //     let bl = model[y + 1][x - 1];
        //     let bm = model[y + 1][x + 0];
        //     let br = model[y + 1][x + 1];
        // }
    };

    let mut part_nums: Vec<u32> = vec![];

    for (y, row) in model.iter().enumerate() {
        let mut digits = vec![];
        let mut is_part_num = false;
        for (x, c) in row.iter().enumerate() {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if is_symbol_adjacent(x, y) {
                        is_part_num = true;
                    }
                    digits.push(c)
                }
                _ => {
                    // println!("number: {:?}", digits);
                    // turn chars into a number
                    if !digits.is_empty() && is_part_num {
                        let n = to_number(&digits);

                        part_nums.push(n);
                        println!("SUM {n}");
                    }

                    // number ended, clear this flag
                    is_part_num = false;
                    digits.clear();
                }
            }
        }
    }

    part_nums.iter().sum()
}

fn to_number(digits: &[&char]) -> u32 {
    digits
        .iter()
        .filter_map(|c| c.to_digit(10))
        .rev()
        .enumerate()
        .map(|(i, d)| d * 10u32.pow(i as u32))
        .collect::<Vec<u32>>()
        .into_iter()
        .rev()
        .sum()
}

pub fn part2(model: Model) -> Answer {
    0
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const INPUT: &str = include_str!("../input/d3");
//     const EXAMPLE: &str = include_str!("../examples/d3");
//
//     // #[test]
//     // fn d3p1_example_test() {
//     //     assert_eq!(
//     //         part1(parse(EXAMPLE.to_string())),
//     //         "put part 1 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d3p1_input_test() {
//     //     assert_eq!(
//     //         part1(parse(INPUT.to_string())),
//     //         "put part 1 final answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d3p2_example_test() {
//     //     assert_eq!(
//     //         part2(parse(EXAMPLE.to_string())),
//     //         "put part 2 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d3p2_input_test() {
//     //     assert_eq!(
//     //         part2(parse(INPUT.to_string())),
//     //         "put part 2 final answer here"
//     //     );
//     // }
// }
