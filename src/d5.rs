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
    fn get_seed(&self, loc: u64) -> u64 {
        let mut n = loc;

        let mut rev = Category::ORDERED;
        rev.reverse();

        for pair in rev.windows(2) {
            let src = pair[0];
            let dst = pair[1];
            n = self.maps.get(&(dst, src)).unwrap().rev_lookup(n);
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
    fn rev_lookup(&self, n: u64) -> u64 {
        self.mappings
            .iter()
            // find the first mapping whose range contained n
            .find_map(|mapping| mapping.rev_lookup(n))
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
    fn rev_lookup(&self, n: u64) -> Option<u64> {
        if self.dst_range.contains(&n) {
            Some(self.src_range.start + n - self.dst_range.start)
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
        .par_bridge()
        .flat_map(|(i, pair)| {
            // println!(
            //     "Getting loc for {} seeds in range {:?}",
            //     pair[1],
            //     (pair[0]..(pair[0] + pair[1]))
            // );
            let seeds = (pair[0]..(pair[0] + pair[1]))
                .into_par_iter()
                .map(|n| almanac.get_loc(n));
            // println!("seeds: {:?}", seeds);
            seeds.collect::<Vec<u64>>()
        })
        .min()
        .unwrap()

    // println!("1");
    // let seed_ranges: Vec<u64> = almanac
    //     .seeds
    //     .chunks(2)
    //     .par_bridge()
    //     .flat_map(|pair| (pair[0]..(pair[0] + pair[1])))
    //     .collect();
    //
    // println!("2");
    // let loc_max: u64 = almanac
    //     .maps
    //     .values()
    //     .filter(|map| map.dst == Category::Location)
    //     // .flat_map(|m| m.mappings.iter().map(|mapping| mapping.dst_range.clone()))
    //     .flat_map(|m| m.mappings.iter().map(|mapping| mapping.dst_range.end))
    //     .par_bridge()
    //     .max()
    //     .unwrap();
    //
    // println!("3");
    // let loc_seeds: Vec<(u64, u64)> = (0..loc_max)
    //     .into_par_iter()
    //     .map(|loc| (loc, almanac.get_seed(loc)))
    //     .collect();
    //
    // println!("4");
    // let mut matching_seeds: Vec<(u64, u64)> = loc_seeds
    //     .into_iter()
    //     .filter(|s| seed_ranges.contains(&s.1))
    //     .collect();
    //
    // println!("5");
    // matching_seeds.sort_by(|(loc1, seed1), (loc2, seed2)| loc1.cmp(loc2));
    //
    // println!("6");
    // matching_seeds.first().unwrap().0
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
    #[test]
    fn d5p2_input_slow_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 77435348);
    }
}
