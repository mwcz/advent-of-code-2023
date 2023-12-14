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

/// Build a new number with the given number of 1 lowest-significance bits.  Ex: bits(3) == 0b111
fn bits(n: u8) -> u128 {
    2u128.pow(n as u32) - 1
}

/// Find the magitude of the leading (highest significance) one in the given number.  Returns None
/// if there are no ones
fn leading_one(n: u128) -> Option<u128> {
    if n == 0 {
        return None;
    }
    let mut n = n;
    let mut count = 0;
    const MASK: u128 = u128::MAX - (u128::MAX >> 1);
    while n & MASK == 0 {
        n <<= 1;
        count += 1;
    }
    Some(127 - count)
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
                // println!("{n:020b} matches {pattern:?}? {matches_pattern}");
                // println!("{broken_n:020b} <- broken");
                // println!("{working_n:020b} <- working");
                // println!(
                //     "{}{condition} <- condition",
                //     " ".repeat(20 - condition.len())
                // );
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
        .map(|(s, p)| ([s.as_str()].repeat(5).join("?").to_string(), p.repeat(5)))
        .collect();

    let count = model
        .iter()
        .map(|(condition, pattern)| {
            let mut local_count = 0;

            let bad_n = buildnum2(condition, '#');
            let good_n = buildnum2(condition, '.');
            let wild_n = buildnum2(condition, '?');

            // println!(
            //     "    cond : {}{condition}",
            //     " ".repeat(100 - condition.len())
            // );
            // println!(" pattern : {pattern:?}");
            // println!("  good_n : {good_n:0100b}");
            // println!("   bad_n : {bad_n:0100b}");
            // println!("  wild_n : {wild_n:0100b}");
            //
            // start with the leftmost digit that is possibly a bad spring
            let start = leading_one(bad_n).max(leading_one(wild_n)).unwrap();

            // println!("   start : {}⬆️", " ".repeat((100 - 1) - (start as usize)));
            //
            // a path to search, consisting of a bit position, the remaining list of contiguous
            // blocks, and a final number representing the discovered broken springs for that path
            type Path<'a> = (u128, &'a [u8], u128);
            // the queue of paths to be searched
            let mut paths: Vec<Path> = vec![(start, &pattern[..], 0)];

            loop {
                if paths.is_empty() {
                    break;
                }
                // println!();

                let (base, pattern, discover) = paths.pop().unwrap();

                // catch errors in discovery so far

                // if discover & (wild_n | bad_n) != discover || !matches_bad_so_far {
                //     local_count -= 1;
                //     continue;
                // }

                // the shortest possible match
                let min_match = (pattern.len() as u8 - 1) + pattern.iter().sum::<u8>();

                // bail if there aren't enough digits left to match the shortest possible solution
                let remaining_digits = base + 1;
                if remaining_digits < min_match as u128 {
                    // println!(
                    //     " EXHAUST : min = {min_match}   base = {base}   rem = {remaining_digits}"
                    // );
                    continue;
                }

                let rest_mask = bits(remaining_digits as u8);
                // mask out the yet-unvisited digits
                let bad_masked = bad_n & !rest_mask;
                let bad_springs_mismatch = bad_masked & !discover != 0;

                // bail if the digits processed so far contain a mismatch
                if bad_springs_mismatch {
                    // println!("discover : {discover:0100b}");
                    // println!("rest_mask: {rest_mask:0100b}");
                    // println!("bad_maskd: {bad_masked:0100b}");
                    // println!("MISMATCH!!!!");
                    continue;
                }

                // println!(" pattern : {:?}", pattern);
                // println!(
                //     "    base : {}⬇️ {}",
                //     " ".repeat((100 - 1) - (base as usize)),
                //     base
                // );
                // println!(
                //     "    cond : {}{condition}",
                //     " ".repeat(100 - condition.len())
                // );
                //
                let pat_n = pattern[0];

                let pat_bits = bits(pat_n);

                // align pat_bits with base
                let pat_bits = pat_bits << ((1 + base) - pat_n as u128);

                // check pat_bits against bad_n and wild_n
                let pat_match = pat_bits & (wild_n | bad_n) == pat_bits;
                // if the first digit is a wild match
                let pat_match_starts_wild = leading_one(pat_bits) == leading_one(pat_bits & wild_n);

                // println!(
                //     "pat_bits : {pat_bits:0100b} {}",
                //     if pat_match { " ✅ " } else { " ✖️ " }
                // );
                //
                // peek left
                let left_mag = 1 << (base + 1);
                let clear_left = discover & left_mag == 0 && bad_n & left_mag == 0;
                // println!(
                //     " < clear : {left_mag:0100b} {}",
                //     if clear_left { " ✅ " } else { " ✖️ " }
                // );

                // peek right
                let clear_right = if let Some(shift) = base.checked_sub(pat_n as u128) {
                    let right_mag = 1 << shift;
                    let clear_right = bad_n & right_mag == 0;
                    // println!(
                    //     " > clear : {right_mag:0100b} {}",
                    //     if clear_right { " ✅ " } else { " ✖️ " }
                    // );
                    clear_right
                } else {
                    // if subtraction fails, it means we fell off the right end of the number, so
                    // there's definitely no broken spring there
                    true
                };
                // println!("discover : {discover:0100b}",);

                let accept_pat = pat_match && clear_left && clear_right;
                // println!("  accept : {accept_pat}");

                if accept_pat {
                    // if the first digit of the match was wild, advance and check the same pattern
                    if base > 0 && pat_match_starts_wild {
                        // println!("  wild s : advance");
                        paths.push((base - 1, pattern, discover));
                    }

                    let new_base = base.checked_sub(pat_n as u128);

                    let new_pats = &pattern[1..];
                    // make sure there are no more broken springs to the right
                    let rest_clear = if let Some(new_base) = new_base {
                        let rest_mask = bits(1 + new_base as u8);
                        // println!("    rest : {rest_mask:0100b}");
                        rest_mask & bad_n == 0
                    } else {
                        true
                    };
                    if new_pats.is_empty() {
                        // println!(" matches : {local_count}");
                        let discover = discover | pat_bits;
                        if rest_clear {
                            // println!("discover : {discover:0100b} WIN",);
                            // println!(
                            //     "    cond : {}{condition}",
                            //     " ".repeat(100 - condition.len())
                            // );
                            local_count += 1;
                        }
                    } else if let Some(new_base) = new_base {
                        paths.push((new_base, new_pats, (discover) | pat_bits));
                    }
                    // if this was a wild match, push another path to check for the _current_
                    // pattern one step to the right
                } else {
                    paths.push((base - 1, pattern, discover));
                }
            }

            local_count
        })
        .sum();

    count
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d12");
    const EXAMPLE: &str = include_str!("../examples/d12");

    #[test]
    fn d12p2_bits_test() {
        assert_eq!(bits(0), 0b0);
        assert_eq!(bits(1), 0b1);
        assert_eq!(bits(2), 0b11);
        assert_eq!(bits(3), 0b111);
        assert_eq!(
            bits(80),
            0b11111111111111111111111111111111111111111111111111111111111111111111111111111111
        );
    }

    #[test]
    fn d12p2_leading_one_test() {
        assert_eq!(leading_one(0b0000), None);
        assert_eq!(leading_one(0b1000), Some(3));
        assert_eq!(leading_one(0b1111), Some(3));
        assert_eq!(leading_one(0b1000001), Some(6));
        assert_eq!(leading_one(0b101010100101000011100), Some(20));
    }
    #[test]
    fn d12p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 21);
    }

    #[test]
    fn d12p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 7694);
    }

    #[test]
    fn d12p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 525152);
    }

    // #[test]
    // fn d12p2_input_test() {
    //     assert_eq!(
    //         part2(parse(INPUT.to_string())),
    //         "put part 2 final answer here"
    //     );
    // }
}
