//! A solution to day 12 year 2023.
//! https://adventofcode.com/2023/day/12

use rayon::prelude::*;

type Model = Vec<(String, Vec<u8>)>;
type Answer = u32;
type Answer2 = u128;

pub fn parse(input: String) -> Model {
    input
        .lines()
        .map(|line| {
            let (springs, counts) = line.trim().split_once(' ').unwrap();
            (
                springs.to_string(),
                counts.split(',').map(|ns| ns.parse().unwrap()).collect(),
            )
        })
        .collect()
}

/// Check number n against a pattern.
fn check(n: u32, pattern: &[u8]) -> bool {
    // println!("checking {n:032b}");
    let mut m = n.reverse_bits();
    let mut shifts = 0;

    // println!("     rev {m:032b}");

    for count in pattern {
        // println!("    bits {count}");

        // consume leading zeroes
        while m & 1 == 0 {
            // println!("    zero {m:032b}");
            m >>= 1;
            shifts += 1;
            if shifts > 32 {
                return false;
            }
        }

        // println!("  zeroed {m:032b}");

        for _ in 0..*count {
            if m & 1 != 1 {
                return false;
            }
            m >>= 1;
            shifts += 1;
            if shifts > 32 {
                return false;
            }
        }

        // require at least one zero after processing ones
        if m & 1 == 1 {
            return false;
        }
    }

    // ensure there are no more ones
    if m > 0 {
        return false;
    }

    true
}

/// Build a number by treating char c as 1 and all other chars as 0.
fn buildnum(s: &str, c: char) -> u32 {
    let mut n = 0;
    for sc in s.trim().chars() {
        if sc == c {
            n |= 1;
        }
        n <<= 1;
        // println!("{n:020b}");
    }
    n >> 1 // shift back to cancel the final looped shift which is unnecessary
}

pub fn part1(model: Model) -> Answer {
    dbg!(&model);
    // use binary to represent springs
    // 1 means broken
    // 0 means working

    // [
    //     0b011101101000,
    //     0b011101100100,
    //     0b011101100010,
    //     0b011101100001,
    //     0b011100110100,
    //     0b011100110010,
    //     0b011100110001,
    //     0b011100011010,
    //     0b011100011001,
    //     0b011100001101,
    // ]

    let mut count = 0;
    let mut total_checks = 0;

    for (condition, pattern) in &model {
        let mut local_count = 0;
        for n in 0..=u32::MAX {
            // stop after searching through 2^20
            if n > 2u32.pow(condition.len() as u32) {
                break;
            }

            total_checks += 1;

            let broken_n = buildnum(condition, '#');
            let working_n = buildnum(condition, '.');
            // println!("  {condition}");
            // println!("{condition_n:020b}");

            let matches_broken = (broken_n & n) == broken_n;
            let matches_working = n & working_n == 0;
            let matches_pattern = check(n, pattern);
            if matches_pattern && matches_broken && matches_working {
                local_count += 1;
                println!("{n:020b} matches {pattern:?}? {matches_pattern}");
                println!("{broken_n:020b} <- broken");
                println!("{working_n:020b} <- working");
                println!(
                    "{}{condition} <- condition",
                    " ".repeat(20 - condition.len())
                );
            }
        }
        // println!("                     {local_count} patterns");
        count += local_count;
        // println!();
    }

    // TODO RESUME HERE
    // this pattern is getting 15 results when it should get 10
    // ?###???????? 3,2,1
    // there probably shouldn't be any extra 1's after the pattern is consumed

    println!("checked {total_checks}");

    count
}

pub fn part2(model: Model) -> Answer {
    // use binary to represent springs
    // 1 means broken
    // 0 means working

    let model: Model = model
        .into_iter()
        .map(|(s, p)| (s.repeat(5), p.repeat(5)))
        .collect();

    let count = model
        .par_iter()
        .map(|(condition, pattern)| {
            let mut local_count = 0;
            for n in (0..=u128::MAX) {
                // stop after searching through 2^20
                if n > 2u128.pow(condition.len() as u32) {
                    break;
                }

                let broken_n = buildnum2(condition, '#');
                let working_n = buildnum2(condition, '.');
                // println!("  {condition}");
                // println!("{condition_n:020b}");

                let matches_broken = (broken_n & n) == broken_n;
                let matches_working = n & working_n == 0;
                let matches_pattern = check2(n, pattern);
                if matches_pattern && matches_broken && matches_working {
                    local_count += 1;
                    println!("{n:020b} matches {pattern:?}? {matches_pattern}");
                    println!("{broken_n:020b} <- broken");
                    println!("{working_n:020b} <- working");
                    println!(
                        "{}{condition} <- condition",
                        " ".repeat(20 - condition.len())
                    );
                }
            }
            local_count
            // println!();
        })
        .sum();

    // TODO RESUME HERE
    // this pattern is getting 15 results when it should get 10
    // ?###???????? 3,2,1
    // there probably shouldn't be any extra 1's after the pattern is consumed

    count
}

/// Check number n against a pattern.
fn check2(n: u128, pattern: &[u8]) -> bool {
    // println!("checking {n:0128b}");
    let mut m = n.reverse_bits();
    let mut shifts = 0;

    // println!("     rev {m:0128b}");

    for count in pattern {
        // println!("    bits {count}");

        // consume leading zeroes
        while m & 1 == 0 {
            // println!("    zero {m:0128b}");
            m >>= 1;
            shifts += 1;
            if shifts > 128 {
                return false;
            }
        }

        // println!("  zeroed {m:0128b}");

        for _ in 0..*count {
            if m & 1 != 1 {
                return false;
            }
            m >>= 1;
            shifts += 1;
            if shifts > 128 {
                return false;
            }
        }

        // require at least one zero after processing ones
        if m & 1 == 1 {
            return false;
        }
    }

    // ensure there are no more ones
    if m > 0 {
        return false;
    }

    true
}

/// Build a number by treating char c as 1 and all other chars as 0.
fn buildnum2(s: &str, c: char) -> u128 {
    let mut n = 0;
    for sc in s.trim().chars() {
        if sc == c {
            n |= 1;
        }
        n <<= 1;
        // println!("{n:020b}");
    }
    n >> 1 // shift back to cancel the final looped shift which is unnecessary
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const INPUT: &str = include_str!("../input/d12");
//     const EXAMPLE: &str = include_str!("../examples/d12");
//
//     // #[test]
//     // fn d12p1_example_test() {
//     //     assert_eq!(
//     //         part1(parse(EXAMPLE.to_string())),
//     //         "put part 1 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d12p1_input_test() {
//     //     assert_eq!(
//     //         part1(parse(INPUT.to_string())),
//     //         "put part 1 final answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d12p2_example_test() {
//     //     assert_eq!(
//     //         part2(parse(EXAMPLE.to_string())),
//     //         "put part 2 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d12p2_input_test() {
//     //     assert_eq!(
//     //         part2(parse(INPUT.to_string())),
//     //         "put part 2 final answer here"
//     //     );
//     // }
// }
