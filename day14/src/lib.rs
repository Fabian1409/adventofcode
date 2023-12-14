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

fn calc_load(input: Vec<Vec<char>>) -> usize {
    input
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .filter_map(|(i, c)| if *c == 'O' { Some(row.len() - i) } else { None })
                .sum::<usize>()
        })
        .sum()
}

fn shift_left(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    input
        .into_iter()
        .map(|row| {
            let mut round_nums = vec![(0, 0)];
            for (i, c) in row.iter().enumerate() {
                match c {
                    'O' => round_nums.last_mut().unwrap().1 += 1,
                    '#' => round_nums.push((i + 1, 0)),
                    _ => {}
                }
            }
            let n = row.len();
            let mut row = vec!['.'; n];
            for (pos, num) in round_nums {
                if pos != 0 {
                    row[pos - 1] = '#';
                }

                for c in row.iter_mut().skip(pos).take(num) {
                    *c = 'O';
                }
            }
            row
        })
        .collect()
}

fn shift_right(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    input
        .into_iter()
        .map(|row| {
            let mut round = vec![(0, 0)];
            for (i, c) in row.iter().rev().enumerate() {
                match c {
                    'O' => round.last_mut().unwrap().1 += 1,
                    '#' => round.push((i + 1, 0)),
                    _ => {}
                }
            }
            let n = row.len();
            let mut row = vec!['.'; n];
            for (pos, num) in round {
                if pos != 0 {
                    row[n - pos] = '#';
                }
                for c in row.iter_mut().take(n - pos).skip(n - pos - num) {
                    *c = 'O';
                }
            }
            row
        })
        .collect()
}

fn cycle(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let north = shift_left(transpose(input));
    let west = shift_left(transpose(north));
    let south = shift_right(transpose(west));
    shift_right(transpose(south))
}

pub struct Day14Solver;

impl<'a> AdventOfCodeDay<'a> for Day14Solver {
    type ParsedInput = Vec<Vec<char>>;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        calc_load(shift_left(transpose(input.to_vec())))
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        let mut possible = HashMap::new();
        let mut cur = input.clone();
        for i in 0..1_000_000_000 {
            cur = cycle(cur);
            if let Some(j) = possible.insert(cur.clone(), i) {
                let cycle_len = i - j;
                let remaining = (1_000_000_000 - i) % cycle_len;

                for _ in 0..remaining - 1 {
                    cur = cycle(cur);
                }

                return calc_load(transpose(cur));
            }
        }
        0
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
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        ";
        assert_eq!(
            Day14Solver::solve_part1(&Day14Solver::parse_input(input.trim())),
            136
        );
    }

    #[test]
    fn test_part2() {
        let input = "
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        ";
        assert_eq!(
            Day14Solver::solve_part2(&Day14Solver::parse_input(input.trim())),
            64
        );
    }
}
