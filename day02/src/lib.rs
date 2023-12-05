#![allow(dead_code)]

use std::{collections::HashMap, error::Error, str::FromStr};

const RED_MAX: usize = 12;
const GREEN_MAX: usize = 13;
const BLUE_MAX: usize = 14;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}
impl FromStr for Color {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err("invalid color".into()),
        }
    }
}

struct Game {
    id: usize,
    turns: Vec<Vec<(usize, Color)>>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (id_str, turns_str) = s.split_once(": ").unwrap();
        let id: usize = id_str.split_once(' ').unwrap().1.parse().unwrap();
        let mut turns = Vec::new();
        for turn_str in turns_str.split("; ") {
            let mut turn = Vec::new();
            for cubes in turn_str.split(", ") {
                let (num, color) = cubes.split_once(' ').unwrap();
                turn.push((num.parse().unwrap(), color.parse().unwrap()))
            }
            turns.push(turn);
        }
        Ok(Game { id, turns })
    }
}

impl Game {
    fn valid(&self) -> bool {
        self.turns.iter().all(|turn| {
            turn.iter().all(|(num, color)| match color {
                Color::Red => num.le(&RED_MAX),
                Color::Green => num.le(&GREEN_MAX),
                Color::Blue => num.le(&BLUE_MAX),
            })
        })
    }

    fn fewest_cubes(&self) -> HashMap<Color, usize> {
        let mut amounts = HashMap::new();

        for turn in self.turns.iter() {
            for (num, color) in turn {
                let entry = amounts.entry(color.clone()).or_insert(0);
                if num > entry {
                    *entry = *num;
                }
            }
        }
        amounts
    }
}

fn part1(input: &str) -> usize {
    input.lines().fold(0, |acc, l| {
        let game: Game = l.parse().unwrap();

        if game.valid() {
            acc + game.id
        } else {
            acc
        }
    })
}

fn part2(input: &str) -> usize {
    input.lines().fold(0, |acc, l| {
        let game: Game = l.parse().unwrap();
        let fewest_cubes = game.fewest_cubes();
        let power: usize = fewest_cubes.values().product();
        acc + power
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";
        assert_eq!(part1(input.trim()), 8);
    }

    #[test]
    fn test_part2() {
        let input = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";
        assert_eq!(part2(input.trim()), 2286);
    }
}
