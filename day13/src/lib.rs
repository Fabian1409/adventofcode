use aoc_traits::AdventOfCodeDay;

fn check_reflection(pattern: &[String], center: (usize, usize)) -> bool {
    (center.0 == 0)
        | (center.1 == pattern.len() - 1)
        | (0..center.0)
            .rev()
            .zip(center.1 + 1..pattern.len())
            .all(|(a, b)| pattern[a] == pattern[b])
}

#[derive(Debug, Clone)]
enum Reflection {
    Horizontal,
    Vertical,
}

impl Reflection {
    fn multiplier(&self) -> usize {
        match self {
            Reflection::Horizontal => 100,
            Reflection::Vertical => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pattern {
    horizontal: Vec<String>,
    vertical: Vec<String>,
}

impl Pattern {
    fn reflection(&self, old: Option<&(usize, Reflection)>) -> Option<(usize, Reflection)> {
        let h_pairs: Vec<_> = self
            .horizontal
            .windows(2)
            .enumerate()
            .filter_map(|(i, w)| if w[0] == w[1] { Some((i, i + 1)) } else { None })
            .collect();

        for pair in h_pairs.iter() {
            if let Some(old) = old {
                if (old.0 == pair.1) & matches!(old.1, Reflection::Horizontal) {
                    continue;
                }
            }
            if check_reflection(&self.horizontal, *pair) {
                return Some((pair.1, Reflection::Horizontal));
            }
        }

        let v_pairs: Vec<_> = self
            .vertical
            .windows(2)
            .enumerate()
            .filter_map(|(i, w)| if w[0] == w[1] { Some((i, i + 1)) } else { None })
            .collect();

        for pair in v_pairs.iter() {
            if let Some(old) = old {
                if (old.0 == pair.1) & matches!(old.1, Reflection::Vertical) {
                    continue;
                }
            }
            if check_reflection(&self.vertical, *pair) {
                return Some((pair.1, Reflection::Vertical));
            }
        }

        None
    }

    fn fix_smudge_reflection(&self) -> usize {
        let reflection = self.reflection(None).unwrap();
        for i in 0..self.horizontal.len() {
            for j in 0..self.horizontal[0].len() {
                let mut pattern = self.clone();
                if pattern.horizontal[i].chars().nth(j).unwrap() == '#' {
                    pattern.horizontal[i].replace_range(j..j + 1, ".");
                } else {
                    pattern.horizontal[i].replace_range(j..j + 1, "#");
                }
                if pattern.vertical[j].chars().nth(i).unwrap() == '#' {
                    pattern.vertical[j].replace_range(i..i + 1, ".");
                } else {
                    pattern.vertical[j].replace_range(i..i + 1, "#");
                }
                if let Some(other) = pattern.reflection(Some(&reflection)) {
                    return other.0 * other.1.multiplier();
                }
            }
        }

        0
    }
}

pub struct Day13Solver;

impl<'a> AdventOfCodeDay<'a> for Day13Solver {
    type ParsedInput = Vec<Pattern>;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        input
            .iter()
            .map(|p| {
                let reflection = p.reflection(None).unwrap();
                reflection.0 * reflection.1.multiplier()
            })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        input.iter().map(|p| p.fix_smudge_reflection()).sum()
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        input
            .split("\n\n")
            .map(|x| {
                let horizontal: Vec<String> = x.lines().map(|l| l.trim().to_string()).collect();
                let vertical = (0..horizontal[0].len())
                    .map(|i| {
                        horizontal
                            .iter()
                            .map(|inner| inner.chars().nth(i).unwrap())
                            .collect()
                    })
                    .collect();
                Pattern {
                    horizontal,
                    vertical,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        ";
        assert_eq!(
            Day13Solver::solve_part1(&Day13Solver::parse_input(input.trim())),
            405
        );
    }

    #[test]
    fn test_part2() {
        let input = "
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        ";
        assert_eq!(
            Day13Solver::solve_part2(&Day13Solver::parse_input(input.trim())),
            400
        );
    }
}
