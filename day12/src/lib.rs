use std::str::FromStr;

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone)]
pub struct Line {
    springs: Vec<char>,
    groups: Vec<usize>,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, groups) = s.split_once(' ').unwrap();
        let springs: Vec<char> = springs.chars().collect();
        let groups = groups.split(',').map(|x| x.parse().unwrap()).collect();
        Ok(Line { springs, groups })
    }
}

fn valid(springs: &[char], groups: &[usize]) -> bool {
    let springs: String = springs.iter().collect();
    let lens: Vec<_> = springs
        .split('.')
        .map(|x| x.len())
        .filter(|x| *x != 0)
        .collect();
    (lens.len() == groups.len()) & groups.iter().zip(lens.iter()).all(|(a, b)| a == b)
}

fn generate_permutations(n: usize, m: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_permutation = String::with_capacity(n);

    fn generate_recursive(
        n: usize,
        m: usize,
        current_permutation: &mut String,
        result: &mut Vec<String>,
    ) {
        if current_permutation.len() == n {
            if current_permutation.chars().filter(|&c| c == '#').count() == m {
                result.push(current_permutation.clone());
            }
            return;
        }

        current_permutation.push('.');
        generate_recursive(n, m, current_permutation, result);
        current_permutation.pop();

        current_permutation.push('#');
        generate_recursive(n, m, current_permutation, result);
        current_permutation.pop();
    }

    generate_recursive(n, m, &mut current_permutation, &mut result);
    result
}

pub struct Day12Solver;

impl<'a> AdventOfCodeDay<'a> for Day12Solver {
    type ParsedInput = Vec<Line>;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        let mut sum = 0;
        for line in input {
            let unknown: Vec<usize> = line
                .springs
                .iter()
                .enumerate()
                .filter_map(|(i, c)| if *c == '?' { Some(i) } else { None })
                .collect();
            let is = line.springs.iter().filter(|c| **c == '#').count();
            let should: usize = line.groups.iter().sum();
            let perms = generate_permutations(unknown.len(), should - is);

            for perm in perms {
                let mut replaced = line.springs.clone();
                for (c, i) in perm.chars().zip(unknown.iter()) {
                    replaced[*i] = c;
                }
                if valid(&replaced, &line.groups) {
                    sum += 1;
                }
            }
        }
        sum
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        todo!()
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        input
            .lines()
            .map(|l| l.trim().parse::<Line>().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        ";
        assert_eq!(
            Day12Solver::solve_part1(&Day12Solver::parse_input(input.trim())),
            21
        );
    }

    #[test]
    fn test_part2() {
        let input = "
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        ";
        assert_eq!(
            Day12Solver::solve_part2(&Day12Solver::parse_input(input.trim())),
            525152
        );
    }
}
