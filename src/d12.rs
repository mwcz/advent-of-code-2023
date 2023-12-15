//! A solution to day 12 year 2023.
//! https://adventofcode.com/2023/day/12

use cached::proc_macro::cached;

type Model = Vec<(String, Vec<u8>)>;
type Answer = u128;

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
    // use binary to represent springs
    // 1 means broken
    // 0 means working

    let count = model
        .iter()
        .map(|(condition, pattern)| {
            let bad_n = charmask(condition, '#');
            let wild_n = charmask(condition, '?');

            // start with the leftmost digit that is possibly a bad spring
            let start = leading_one(bad_n).max(leading_one(wild_n)).unwrap();

            solve((start, bad_n, wild_n, pattern, 0, condition.clone(), 0))
        })
        .sum();

    count
}

fn has_room(pattern: &[u8], base: u128) -> bool {
    let remaining_digits = base + 1;
    let min_match = (pattern.len() as u8 - 1) + pattern.iter().sum::<u8>();

    remaining_digits >= min_match as u128
}

#[cached(
    key = "String",
    convert = r#"{ format!("{base}-{bad_n}-{wild_n}-{:?}-{}",pattern, discover & (1 << (base + 1)))  }"#
)]
fn solve(
    (base, bad_n, wild_n, pattern, discover, condition, last_bits_added): (
        u128,
        u128,
        u128,
        &[u8],
        u128,
        String,
        u128,
    ),
) -> u128 {
    // bail if there aren't enough digits left to match the shortest possible solution
    if !has_room(pattern, base) {
        return 0;
    }

    let rest_mask = bits(1 + base as u8);
    // // mask out the yet-unvisited digits
    let bad_masked = bad_n & !rest_mask;
    let bad_springs_mismatch = bad_masked & !discover != 0;

    // bail if the digits processed so far contain a mismatch
    if bad_springs_mismatch {
        return 0;
    }

    let pat_n = pattern[0];

    let pat_bits = bits(pat_n);

    // align pat_bits with base
    let pat_bits = pat_bits << ((1 + base) - pat_n as u128);

    // check pat_bits against bad_n and wild_n
    let pat_match = pat_bits & (wild_n | bad_n) == pat_bits;
    // if the first digit is a wild match
    let pat_match_starts_wild = leading_one(pat_bits) == leading_one(pat_bits & wild_n);

    // peek left
    let left_mag = 1 << (base + 1);
    let clear_left = last_bits_added & left_mag == 0 && bad_n & left_mag == 0;

    // peek right
    let clear_right = if let Some(shift) = base.checked_sub(pat_n as u128) {
        let right_mag = 1 << shift;
        let clear_right = bad_n & right_mag == 0;
        clear_right
    } else {
        // if subtraction fails, it means we fell off the right end of the number, so
        // there's definitely no broken spring there
        true
    };

    let accept_pat = pat_match && clear_left && clear_right;

    let mut win = false;
    let mut step_same = false;
    let mut step_progress = None;

    if accept_pat {
        // if the first digit of the match was wild, advance and check the same pattern
        if base > 0 && pat_match_starts_wild {
            step_same = true;
        }

        let new_base = base.checked_sub(pat_n as u128);
        let new_pats = &pattern[1..];

        // make sure there are no more broken springs to the right and that all bad springs in the
        // pattern have been discovered
        let rest_clear = if let Some(new_base) = new_base {
            let rest_mask = bits(1 + new_base as u8);

            let rest_clear = rest_mask & bad_n == 0;

            rest_clear
        } else {
            true
        };

        if new_pats.is_empty() {
            if rest_clear {
                win = true;
            }
        } else if let Some(new_base) = new_base {
            step_progress = Some((new_base, new_pats, (discover) | pat_bits));
        }
    } else {
        step_same = true;
    }

    // wait for user to press enter
    // std::io::stdin().read_line(&mut String::new()).unwrap();
    // std::thread::sleep_ms(500);

    let win_score = if win { 1 } else { 0 };

    let step_progress_score = if let Some((new_base, new_pattern, new_discover)) = step_progress {
        solve((
            new_base,
            bad_n,
            wild_n,
            new_pattern,
            new_discover,
            condition.clone(),
            pat_bits,
        ))
    } else {
        0
    };

    let step_same_score = if step_same {
        if let Some(new_base) = base.checked_sub(1) {
            if has_room(pattern, new_base) {
                solve((
                    base - 1,
                    bad_n,
                    wild_n,
                    pattern,
                    discover,
                    condition.clone(),
                    0,
                ))
            } else {
                0
            }
        } else {
            0
        }
    } else {
        0
    };

    win_score + step_progress_score + step_same_score
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
            let bad_n = charmask(condition, '#');
            let wild_n = charmask(condition, '?');

            // start with the leftmost digit that is possibly a bad spring
            let start = leading_one(bad_n).max(leading_one(wild_n)).unwrap();

            solve((start, bad_n, wild_n, pattern, 0, condition.clone(), 0))
        })
        .sum();

    count
}

/// Build a number from a string by treating given char c as 1 and all other chars as 0.
fn charmask(s: &str, c: char) -> u128 {
    let mut n = 0;
    for sc in s.trim().chars() {
        if sc == c {
            n |= 1;
        }
        n <<= 1;
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

    #[test]
    fn d12p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 5071883216318);
    }
}
