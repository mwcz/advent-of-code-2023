//! A solution to day 7 year 2023.
//! https://adventofcode.com/2023/day/7

use std::{cmp::Ordering, str::FromStr};

type Model = String;
type Answer = usize;

pub fn parse(input: String) -> Model {
    input
}

pub fn part1(input: Model) -> Answer {
    let mut hands: Vec<Hand> = input.lines().map(Hand::from_part1).collect();
    hands.sort_by(|a, b| match a._type.partial_cmp(&b._type) {
        Some(ord) => match ord {
            Ordering::Equal => {
                // compare hand card by card
                for (card_a, card_b) in a.cards.iter().zip(&b.cards) {
                    match card_a.cmp(card_b) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {}
                    }
                }

                Ordering::Equal
            }
            _ => ord,
        },
        None => panic!("can't compare hand types!"),
    });

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

pub fn part2(input: Model) -> Answer {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|h| {
            let t = Hand::from_part2(h);
            if t.cards.contains(&1) {
                println!("{h} -> {t:?}");
            }
            t
        })
        .collect();
    hands.sort_by(|a, b| match a._type.partial_cmp(&b._type) {
        Some(ord) => match ord {
            Ordering::Equal => {
                // hand types are equal, so compare hand card by card
                for (card_a, card_b) in a.cards.iter().zip(&b.cards) {
                    match card_a.cmp(card_b) {
                        Ordering::Less => {
                            println!("{:?} < {:?}\n", a.cards, b.cards);
                            return Ordering::Less;
                        }
                        Ordering::Greater => {
                            println!("{:?} < {:?}\n", b.cards, a.cards);
                            return Ordering::Greater;
                        }
                        Ordering::Equal => {}
                    }
                }

                Ordering::Equal
            }
            _ => ord,
        },
        None => panic!("can't compare hand types!"),
    });

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

#[derive(Debug)]
pub struct Hand {
    _type: HandType,
    cards: Vec<u8>,
    bid: usize,
}

impl Hand {
    fn from_part1(value: &str) -> Self {
        let (hand_str, bid_str) = value.split_once(' ').unwrap();
        let cards: Vec<u8> = hand_str.chars().map(to_card).collect();
        let types = HandType::from_cards(&cards);
        Hand {
            cards,
            bid: bid_str.parse().unwrap(),
            _type: types,
        }
    }

    fn from_part2(value: &str) -> Self {
        let (hand_str, bid_str) = value.split_once(' ').unwrap();
        let cards: Vec<u8> = hand_str
            .chars()
            .map(to_card)
            .map(|c| if c == 11 { 1 } else { c })
            .collect();
        let types = HandType::from_cards_with_jokers(&cards);
        Hand {
            cards,
            bid: bid_str.parse().unwrap(),
            _type: types,
        }
    }
}

fn to_card(c: char) -> u8 {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("invalid card"),
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
enum HandType {
    // variant order matters
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: &[u8]) -> HandType {
        let mut cards = cards.to_vec();
        cards.sort();
        let groups: Vec<&[u8]> = cards.group_by(|a, b| a == b).collect();

        let is_five = groups.len() == 1;
        if is_five {
            return HandType::FiveOfAKind;
        }

        let is_four = groups.iter().any(|group| group.len() == 4);
        if is_four {
            return HandType::FourOfAKind;
        }

        let is_full_house = groups.len() == 2 && (groups[0].len() == 2 || groups[1].len() == 2);
        if is_full_house {
            return HandType::FullHouse;
        }

        let is_three = groups.iter().any(|group| group.len() == 3);
        if is_three {
            return HandType::ThreeOfAKind;
        }

        let pairs = groups.iter().filter(|group| group.len() == 2).count();
        let is_two_pair = pairs == 2;
        if is_two_pair {
            return HandType::TwoPair;
        }

        let is_one_pair = pairs == 1;
        if is_one_pair {
            return HandType::OnePair;
        }

        HandType::HighCard
    }

    fn from_cards_with_jokers(cards: &[u8]) -> HandType {
        let mut cards = cards.to_vec();
        cards.sort();

        let jokers = cards.iter().filter(|&&c| c == 1).count();
        let without_jokers: Vec<u8> = cards.iter().filter(|&&c| c != 1).copied().collect();
        let groups: Vec<&[u8]> = without_jokers.group_by(|a, b| a == b).collect();

        let maxlen = groups.iter().map(|group| group.len()).max().unwrap_or(0);

        let is_five = maxlen + jokers == 5;
        if is_five {
            return HandType::FiveOfAKind;
        }

        let is_four = maxlen + jokers >= 4;
        if is_four {
            return HandType::FourOfAKind;
        }

        let is_full_house = match (groups.len(), jokers) {
            (1, _) if jokers > 1 => true,
            (2, _) if jokers > 0 => true,
            (3, _) => false,
            _ => groups.len() == 2 && (groups[0].len() == 2 || groups[1].len() == 2),
        };
        if is_full_house {
            return HandType::FullHouse;
        }

        let is_three = maxlen + jokers >= 3;
        if is_three {
            return HandType::ThreeOfAKind;
        }

        let is_two_pair = maxlen == 2 && jokers >= 1;
        if is_two_pair {
            return HandType::TwoPair;
        }

        let is_one_pair = maxlen + jokers == 2;
        if is_one_pair {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d7");
    const EXAMPLE: &str = include_str!("../examples/d7");

    // #[test]
    // fn d7p1_example_test() {
    //     assert_eq!(
    //         part1(parse(EXAMPLE.to_string())),
    //         "put part 1 example answer here"
    //     );
    // }
    //
    // #[test]
    // fn d7p1_input_test() {
    //     assert_eq!(
    //         part1(parse(INPUT.to_string())),
    //         "put part 1 final answer here"
    //     );
    // }
    //
    // #[test]
    // fn d7p2_example_test() {
    //     assert_eq!(
    //         part2(parse(EXAMPLE.to_string())),
    //         "put part 2 example answer here"
    //     );
    // }
    //
    // #[test]
    // fn d7p2_input_test() {
    //     assert_eq!(
    //         part2(parse(INPUT.to_string())),
    //         "put part 2 final answer here"
    //     );
    // }
}
