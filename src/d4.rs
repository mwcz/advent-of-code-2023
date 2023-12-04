//! A solution to day 4 year 2023.
//! https://adventofcode.com/2023/day/4

type Model = Vec<(Vec<u32>, Vec<u32>)>;
type Answer = u32;

pub fn parse(input: String) -> Model {
    input
        .lines()
        .map(|line| {
            let (name, nums) = line.split_once(':').unwrap();
            let (wins, haves) = nums.split_once('|').unwrap();
            let wins = wins
                .split_whitespace()
                .map(|n| n.trim().parse().unwrap())
                .collect();
            let haves = haves
                .split_whitespace()
                .map(|n| n.trim().parse().unwrap())
                .collect();
            (wins, haves)
        })
        .collect()
}

pub fn part1(cards: Model) -> Answer {
    let mut total_score = 0;
    for (wins, haves) in cards {
        let card_score = haves.iter().filter(|h| wins.contains(h)).count() as u32;
        if card_score > 0 {
            total_score += 2u32.pow(card_score - 1);
        }
    }

    total_score
}

pub fn part2(mut cards: Model) -> Answer {
    #[derive(Clone)]
    struct Card {
        wins: Vec<u32>,
        haves: Vec<u32>,
        matches: usize,
        copies: u32,
    }

    let mut cards: Vec<Card> = cards
        .into_iter()
        .map(|(wins, haves)| Card {
            matches: haves.iter().filter(|h| wins.contains(h)).count(),
            wins,
            haves,
            copies: 1,
        })
        .collect();

    let mut i = 0;
    loop {
        let matches = cards[i].matches;
        let copies = cards[i].copies;

        for j in 1..=matches {
            if let Some(c) = cards.get_mut(i + j) {
                c.copies += copies;
            }
        }

        i += 1;
        if i == cards.len() {
            break;
        }
    }

    cards.iter().map(|c| c.copies).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d4");
    const EXAMPLE: &str = include_str!("../examples/d4");

    #[test]
    fn d4p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 13);
    }

    #[test]
    fn d4p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 33950);
    }

    #[test]
    fn d4p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 30);
    }

    #[test]
    fn d4p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 14814534);
    }
}
