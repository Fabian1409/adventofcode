use std::str::FromStr;

use aoc_traits::AdventOfCodeDay;

fn hash(string: &str) -> usize {
    let mut cur = 0;
    for c in string.chars() {
        cur += c as usize;
        cur *= 17;
        cur %= 256;
    }
    cur
}

enum Op {
    Remove,
    Insert(usize),
}

struct Operation {
    hash: usize,
    label: String,
    op: Op,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('-') {
            let label = s.strip_suffix('-').unwrap();
            Ok(Operation {
                hash: hash(label),
                label: label.to_owned(),
                op: Op::Remove,
            })
        } else {
            let (label, f) = s.split_once('=').unwrap();
            Ok(Operation {
                hash: hash(label),
                label: label.to_owned(),
                op: Op::Insert(f.parse().unwrap()),
            })
        }
    }
}

#[derive(Clone)]
struct Lense {
    label: String,
    f: usize,
}

pub struct Day15Solver;

impl<'a> AdventOfCodeDay<'a> for Day15Solver {
    type ParsedInput = &'a str;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        input.split(',').map(hash).sum()
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        let mut boxes = vec![Vec::<Lense>::new(); 256];
        let ops: Vec<_> = input
            .split(',')
            .map(|s| s.parse::<Operation>().unwrap())
            .collect();
        for op in ops {
            match op.op {
                Op::Remove => {
                    if let Some(pos) = boxes[op.hash].iter().position(|l| l.label == op.label) {
                        boxes[op.hash].remove(pos);
                    }
                }
                Op::Insert(f) => {
                    if let Some(pos) = boxes[op.hash].iter().position(|l| l.label == op.label) {
                        boxes.get_mut(op.hash).unwrap().get_mut(pos).unwrap().f = f;
                    } else {
                        boxes[op.hash].push(Lense { label: op.label, f });
                    }
                }
            }
        }

        boxes
            .into_iter()
            .enumerate()
            .map(|(i, b)| {
                b.into_iter()
                    .enumerate()
                    .map(|(j, l)| (i + 1) * (j + 1) * l.f)
                    .sum::<usize>()
            })
            .sum()
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        input
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(
            Day15Solver::solve_part1(&Day15Solver::parse_input(input.trim())),
            1320
        );
    }

    #[test]
    fn test_part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(
            Day15Solver::solve_part2(&Day15Solver::parse_input(input.trim())),
            145
        );
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("rn"), 0);
    }
}
