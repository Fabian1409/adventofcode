use std::collections::HashMap;

use aoc_traits::AdventOfCodeDay;

struct Part {
    number: usize,
    start: usize,
    end: usize,
}

fn parse_part(line: &[char], index: usize) -> Part {
    let mut digits = String::new();
    let mut start = index;
    let mut end = index;
    for c in line.iter().take(index).rev() {
        if c.is_ascii_digit() {
            start -= 1;
        } else {
            break;
        }
    }
    for c in line.iter().skip(index) {
        if c.is_ascii_digit() {
            end += 1;
        } else {
            break;
        }
    }
    for c in line.iter().skip(start).take(end - start) {
        digits.push(*c)
    }
    let number = digits.parse().unwrap();
    Part { number, start, end }
}

pub struct Day03Solver;

impl<'a> AdventOfCodeDay<'a> for Day03Solver {
    type ParsedInput = Vec<Vec<char>>;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        let mut parts = HashMap::new();

        for (i, line) in input.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if (*c != '.') & !c.is_ascii_digit() {
                    for i_off in 0..=2 {
                        for j_off in 0..=2 {
                            let x = (i + i_off).saturating_sub(1).min(input.len() - 1);
                            let y = (j + j_off).saturating_sub(1).min(line.len() - 1);
                            let c = input[x][y];
                            if c.is_ascii_digit() {
                                let part = parse_part(&input[x], y);
                                parts.insert((part.start..part.end, x), part.number);
                            }
                        }
                    }
                }
            }
        }
        parts.values().sum()
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        let mut sum = 0;

        for (i, line) in input.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == '*' {
                    let mut gears = HashMap::new();
                    for i_off in 0..=2 {
                        for j_off in 0..=2 {
                            let x = (i + i_off).saturating_sub(1).min(input.len() - 1);
                            let y = (j + j_off).saturating_sub(1).min(input[0].len() - 1);
                            let c = input[x][y];
                            if c.is_ascii_digit() {
                                let part = parse_part(&input[x], y);
                                gears.insert((part.start..part.end, x), part.number);
                            }
                        }
                    }

                    if gears.len() == 2 {
                        sum += gears.values().product::<usize>();
                    }
                }
            }
        }
        sum
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
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        ";
        assert_eq!(
            Day03Solver::solve_part1(&Day03Solver::parse_input(input.trim())),
            4361
        );
    }

    #[test]
    fn test_part2() {
        let input = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        ";
        assert_eq!(
            Day03Solver::solve_part2(&Day03Solver::parse_input(input.trim())),
            467835
        );
    }
}
