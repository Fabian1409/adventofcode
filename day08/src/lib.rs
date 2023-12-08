use std::collections::HashMap;

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone)]
struct Node {
    label: String,
    left: String,
    right: String,
}

pub struct Input {
    dirs: Vec<char>,
    graph: HashMap<String, Node>,
}

pub struct Day08Solver;

impl<'a> AdventOfCodeDay<'a> for Day08Solver {
    type ParsedInput = Input;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        let mut num = 0;
        let mut cur = input.graph.get("AAA").unwrap();
        let mut dir_idx = 0;
        while cur.label.ne("ZZZ") {
            let dir = input.dirs[dir_idx];
            cur = match dir {
                'L' => input.graph.get(&cur.left).unwrap(),
                _ => input.graph.get(&cur.right).unwrap(),
            };
            dir_idx = (dir_idx + 1) % input.dirs.len();
            num += 1;
        }
        num
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        todo!()
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        let (dirs, nodes) = input.split_once("\n\n").unwrap();
        let dirs = dirs.trim().chars().collect();
        let graph = nodes
            .lines()
            .map(|l| {
                let (label, children) = l.trim().split_once(" = ").unwrap();
                let (left, right) = children[1..children.len() - 1].split_once(", ").unwrap();
                (
                    label.to_owned(),
                    Node {
                        label: label.to_owned(),
                        left: left.to_owned(),
                        right: right.to_owned(),
                    },
                )
            })
            .collect();

        Input { dirs, graph }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        ";
        assert_eq!(
            Day08Solver::solve_part1(&Day08Solver::parse_input(input.trim())),
            2
        );
    }

    #[test]
    fn test_part2() {
        let input = "
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        ";
        assert_eq!(
            Day08Solver::solve_part2(&Day08Solver::parse_input(input.trim())),
            6
        );
    }
}
