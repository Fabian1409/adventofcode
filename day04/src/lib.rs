#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Clone, Debug)]
struct Card {
    numbers: HashSet<usize>,
    winning: HashSet<usize>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let offset = s.find(": ").unwrap() + ": ".len();
        let (winning, numbers) = s[offset..].split_once(" | ").unwrap();
        Ok(Card {
            winning: winning
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
            numbers: numbers
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
        })
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.trim().parse::<Card>().unwrap())
        .fold(0, |acc, card| {
            let intersection_len = card.numbers.intersection(&card.winning).count();
            if intersection_len != 0 {
                acc + 2usize.pow(intersection_len as u32 - 1)
            } else {
                acc
            }
        })
}

fn part2(input: &str) -> usize {
    let mut card_copies: HashMap<usize, usize> = HashMap::new();
    let mut sum = 0;

    for (i, card) in input
        .lines()
        .map(|l| l.trim().parse::<Card>().unwrap())
        .enumerate()
    {
        sum += 1;
        let n = card.numbers.intersection(&card.winning).count();
        for j in 0..n {
            *card_copies.entry(i + j + 1).or_default() += 1;
        }

        let copies = card_copies.get(&i).cloned().unwrap_or(0);

        if copies != 0 {
            sum += copies;
            for j in 0..n {
                *card_copies.entry(i + j + 1).or_default() += copies;
            }
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_part1() {
        let input = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";
        assert_eq!(part1(input.trim()), 13);
    }

    #[test]
    fn test_part2() {
        let input = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";

        assert_eq!(part2(input.trim()), 30);
    }
}
