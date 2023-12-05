#![allow(dead_code)]

use std::{collections::HashSet, ops::Range, str::FromStr};

struct Almanac {
    seeds: HashSet<u64>,
    mappings: Vec<Vec<(Range<u64>, Range<u64>)>>,
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seeds, rest) = s.split_once("\n\n").unwrap();
        let seeds: HashSet<u64> = seeds
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();

        let mappings: Vec<Vec<(Range<u64>, Range<u64>)>> = rest
            .split("\n\n")
            .map(|p| {
                let (_, lines) = p.split_once(':').unwrap();
                let mut mapping = Vec::new();
                for l in lines.trim().split('\n') {
                    let mut iter = l.trim().split(' ').map(|x| x.parse().unwrap());
                    let dst_start = iter.next().unwrap();
                    let src_start = iter.next().unwrap();
                    let len = iter.next().unwrap();
                    mapping.push((src_start..src_start + len, dst_start..dst_start + len));
                }
                mapping
            })
            .collect();

        Ok(Almanac { seeds, mappings })
    }
}

impl Almanac {
    fn seed_to_location(&self, seed: u64) -> u64 {
        let mut next = seed;

        for mapping in self.mappings.iter() {
            next = match mapping.iter().find(|(src, _)| src.contains(&next)) {
                Some((src, dst)) => dst.start + next - src.start,
                None => next,
            };
        }

        next
    }
}

fn part1(input: &str) -> u64 {
    let almanac = input.parse::<Almanac>().unwrap();
    almanac
        .seeds
        .iter()
        .map(|seed| almanac.seed_to_location(*seed))
        .min()
        .unwrap()
}

fn part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        ";
        assert_eq!(part1(input.trim()), 35);
    }
}
