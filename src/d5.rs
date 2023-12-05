//! A solution to day 5 year 2023.
//! https://adventofcode.com/2023/day/5

use memoize::memoize;
use rayon::prelude::*;
use std::{cell::RefCell, collections::HashMap, ops::Range, panic::Location};

type Model = Almanac;
type Answer = u64;

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<(Category, Category), Map>,
}

impl Almanac {
    fn get_loc(&self, seed: u64) -> u64 {
        let mut n = seed;

        for pair in Category::ORDERED.windows(2) {
            let src = pair[0];
            let dst = pair[1];
            n = self.maps.get(&(src, dst)).unwrap().lookup(n);
        }
        n
    }
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let mut chunks = value.split("\n\n");
        let seeds = chunks.next().unwrap()[7..]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let maps = chunks
            .map(|chunk| {
                let map: Map = chunk.into();
                ((map.src, map.dst), map)
            })
            .collect();

        Almanac { seeds, maps }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Map {
    src: Category,
    dst: Category,
    mappings: Vec<Mapping>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut lines = value.trim().lines();
        let categories: Vec<&str> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .split('-')
            .collect();

        let src = categories[0].into();
        let dst = categories[2].into();

        let mappings = lines.map(Mapping::from).collect();

        Map { src, dst, mappings }
    }
}

impl Map {
    fn lookup(&self, n: u64) -> u64 {
        self.mappings
            .iter()
            // find the first mapping whose range contained n
            .find_map(|mapping| mapping.lookup(n))
            .unwrap_or(n)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Mapping {
    src_range: Range<u64>,
    dst_range: Range<u64>,
}

impl Mapping {
    fn lookup(&self, n: u64) -> Option<u64> {
        if self.src_range.contains(&n) {
            let offset = n - self.src_range.start;
            Some(self.dst_range.start + offset)
        } else {
            None
        }
    }
}

impl From<&str> for Mapping {
    fn from(value: &str) -> Self {
        let mut nums = value.split_whitespace().map(|n| n.parse().unwrap());
        let dst_start = nums.next().unwrap();
        let src_start = nums.next().unwrap();
        let len = nums.next().unwrap();

        Mapping {
            src_range: src_start..(src_start + len),
            dst_range: dst_start..(dst_start + len),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl Category {
    const ORDERED: [Self; 8] = [
        Category::Seed,
        Category::Soil,
        Category::Fertilizer,
        Category::Water,
        Category::Light,
        Category::Temperature,
        Category::Humidity,
        Category::Location,
    ];
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        match value {
            "seed" => Category::Seed,
            "soil" => Category::Soil,
            "fertilizer" => Category::Fertilizer,
            "water" => Category::Water,
            "light" => Category::Light,
            "temperature" => Category::Temperature,
            "humidity" => Category::Humidity,
            "location" => Category::Location,
            _ => unreachable!(),
        }
    }
}

pub fn parse(input: String) -> Model {
    Almanac::from(input.as_str())
}

pub fn part1(almanac: Model) -> Answer {
    almanac
        .seeds
        .iter()
        .map(|&s| almanac.get_loc(s))
        .min()
        .unwrap()
}

pub fn part2(almanac: Model) -> Answer {
    almanac
        .seeds
        .chunks(2)
        .enumerate()
        .flat_map(|(i, pair)| {
            println!(
                "Getting loc for {} seeds in range {:?}",
                pair[1],
                (pair[0]..(pair[0] + pair[1]))
            );
            (pair[0]..(pair[0] + pair[1])).map(|n| almanac.get_loc(n))
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d5");
    const EXAMPLE: &str = include_str!("../examples/d5");

    #[test]
    fn d5p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 35);
    }

    #[test]
    fn d5p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 910845529);
    }

    #[test]
    fn d5p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 46);
    }

    // commented to prevent it from being included when running all tests because it's too inefficient
    // #[test]
    // fn d5p2_input_test() {
    //     assert_eq!(part2(parse(INPUT.to_string())), 77435348);
    // }
}
