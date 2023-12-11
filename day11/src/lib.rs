use std::collections::HashMap;

use aoc_traits::AdventOfCodeDay;

fn transpose(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<char>>()
        })
        .collect()
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

pub struct Day11Solver;

impl<'a> AdventOfCodeDay<'a> for Day11Solver {
    type ParsedInput = Vec<Vec<char>>;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        let mut y_expansions = HashMap::new();
        for (i, row) in input.iter().enumerate() {
            if row.iter().all(|c| *c == '.') {
                y_expansions.insert(i, 2);
            }
        }

        let transposed = transpose(input.to_vec());
        let mut x_expansions = HashMap::new();
        for (i, row) in transposed.iter().enumerate() {
            if row.iter().all(|c| *c == '.') {
                x_expansions.insert(i, 2);
            }
        }

        let galaxies: Vec<_> = input
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, c)| if *c == '#' { Some((j, i)) } else { None })
                    .collect::<Vec<_>>()
            })
            .collect();

        galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, galaxy)| {
                galaxies
                    .iter()
                    .skip(i + 1)
                    .map(|other| {
                        let x_adj_galaxy: usize = (0..galaxy.0)
                            .map(|x| x_expansions.get(&x).unwrap_or(&1))
                            .sum();
                        let y_adj_galaxy: usize = (0..galaxy.1)
                            .map(|y| y_expansions.get(&y).unwrap_or(&1))
                            .sum();
                        let x_adj_other: usize = (0..other.0)
                            .map(|x| x_expansions.get(&x).unwrap_or(&1))
                            .sum();
                        let y_adj_other: usize = (0..other.1)
                            .map(|y| y_expansions.get(&y).unwrap_or(&1))
                            .sum();
                        distance((x_adj_galaxy, y_adj_galaxy), (x_adj_other, y_adj_other))
                    })
                    .collect::<Vec<_>>()
            })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        let mut y_expansions = HashMap::new();
        for (i, row) in input.iter().enumerate() {
            if row.iter().all(|c| *c == '.') {
                y_expansions.insert(i, 1_000_000);
            }
        }

        let transposed = transpose(input.to_vec());
        let mut x_expansions = HashMap::new();
        for (i, row) in transposed.iter().enumerate() {
            if row.iter().all(|c| *c == '.') {
                x_expansions.insert(i, 1_000_000);
            }
        }

        let galaxies: Vec<_> = input
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, c)| if *c == '#' { Some((j, i)) } else { None })
                    .collect::<Vec<_>>()
            })
            .collect();

        galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, galaxy)| {
                galaxies
                    .iter()
                    .skip(i + 1)
                    .map(|other| {
                        let x_adj_galaxy: usize = (0..galaxy.0)
                            .map(|x| x_expansions.get(&x).unwrap_or(&1))
                            .sum();
                        let y_adj_galaxy: usize = (0..galaxy.1)
                            .map(|y| y_expansions.get(&y).unwrap_or(&1))
                            .sum();
                        let x_adj_other: usize = (0..other.0)
                            .map(|x| x_expansions.get(&x).unwrap_or(&1))
                            .sum();
                        let y_adj_other: usize = (0..other.1)
                            .map(|y| y_expansions.get(&y).unwrap_or(&1))
                            .sum();
                        distance((x_adj_galaxy, y_adj_galaxy), (x_adj_other, y_adj_other))
                    })
                    .collect::<Vec<_>>()
            })
            .sum()
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        input.lines().map(|l| l.trim().chars().collect()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        ";
        assert_eq!(
            Day11Solver::solve_part1(&Day11Solver::parse_input(input.trim())),
            374
        );
    }

    #[test]
    fn test_part2() {
        let input = "
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        ";
        assert_eq!(
            Day11Solver::solve_part2(&Day11Solver::parse_input(input.trim())),
            82_000_210
        );
    }
}
