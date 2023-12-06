use std::str::FromStr;

use aoc_traits::AdventOfCodeDay;

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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

pub struct Game {
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

    fn fewest_cubes_power(&self) -> usize {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for turn in self.turns.iter() {
            for (num, color) in turn {
                match color {
                    Color::Red => red = red.max(*num),
                    Color::Green => green = green.max(*num),
                    Color::Blue => blue = blue.max(*num),
                }
            }
        }

        red * green * blue
    }
}

pub struct Day02Solver;

impl<'a> AdventOfCodeDay<'a> for Day02Solver {
    type ParsedInput = Vec<Game>;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        input.iter().fold(
            0,
            |acc, game| if game.valid() { acc + game.id } else { acc },
        )
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        input
            .iter()
            .fold(0, |acc, game| acc + game.fewest_cubes_power())
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        input.lines().map(|l| l.parse().unwrap()).collect()
    }
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
        assert_eq!(
            Day02Solver::solve_part1(&Day02Solver::parse_input(input.trim())),
            8
        );
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
        assert_eq!(
            Day02Solver::solve_part2(&Day02Solver::parse_input(input.trim())),
            2286
        );
    }
}
