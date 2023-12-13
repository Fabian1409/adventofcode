use std::{collections::HashMap, str::FromStr};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone)]
pub struct Line {
    springs: Vec<char>,
    groups: Vec<usize>,
}

impl Line {
    fn unfold(&self) -> Self {
        let mut springs = Vec::new();
        let mut groups = Vec::new();
        for _ in 0..4 {
            springs.extend_from_slice(&self.springs);
            springs.push('?');
            groups.extend_from_slice(&self.groups);
        }
        springs.extend_from_slice(&self.springs);
        groups.extend_from_slice(&self.groups);

        Line { springs, groups }
    }
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

fn arrangements(springs: &[char], groups: &[usize]) -> usize {
    let mut cache = HashMap::new();
    dfs(&mut cache, springs, groups, 0, 0, 0)
}

fn dfs(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    springs: &[char],
    groups: &[usize],
    from: usize,
    group: usize,
    size: usize,
) -> usize {
    if from >= springs.len() {
        if group >= groups.len() {
            return 1;
        }

        if group == groups.len() - 1 && groups[group] == size {
            return 1;
        }

        return 0;
    }

    match springs[from] {
        '.' => {
            if size == 0 {
                return dfs(cache, springs, groups, from + 1, group, size);
            }

            if group >= groups.len() || size != groups[group] {
                return 0;
            }

            dfs(cache, springs, groups, from + 1, group + 1, 0)
        }
        '#' => {
            if group >= groups.len() || size + 1 > groups[group] {
                return 0;
            }

            dfs(cache, springs, groups, from + 1, group, size + 1)
        }
        _ => {
            if let Some(answer) = cache.get(&(from, group, size)).copied() {
                return answer;
            }

            let mut ways = 0;

            if size == 0 {
                ways += dfs(cache, springs, groups, from + 1, group, size);
            }

            if group < groups.len() && size < groups[group] {
                ways += dfs(cache, springs, groups, from + 1, group, size + 1);
            }

            if group < groups.len() && size == groups[group] {
                ways += dfs(cache, springs, groups, from + 1, group + 1, 0);
            }

            cache.insert((from, group, size), ways);
            ways
        }
    }
}

pub struct Day12Solver;

impl<'a> AdventOfCodeDay<'a> for Day12Solver {
    type ParsedInput = Vec<Line>;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        input
            .iter()
            .map(|l| arrangements(&l.springs, &l.groups))
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        input
            .iter()
            .map(|l| l.unfold())
            .map(|l| arrangements(&l.springs, &l.groups))
            .sum()
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
