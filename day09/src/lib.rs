use aoc_traits::AdventOfCodeDay;

pub struct Day09Solver;

fn get_diffs(history: Vec<i32>) -> Vec<Vec<i32>> {
    let mut diffs = vec![history];
    loop {
        let diff: Vec<_> = diffs
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();
        if diff.iter().all(|x| *x == 0) {
            break;
        } else {
            diffs.push(diff);
        }
    }
    diffs
}

impl<'a> AdventOfCodeDay<'a> for Day09Solver {
    type ParsedInput = Vec<Vec<i32>>;

    type Part1Output = i32;

    type Part2Output = i32;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        input
            .iter()
            .map(|history| {
                get_diffs(history.clone())
                    .iter()
                    .rev()
                    .fold(0, |acc, e| acc + e.last().unwrap())
            })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        input
            .iter()
            .map(|history| {
                get_diffs(history.clone())
                    .iter()
                    .rev()
                    .fold(0, |acc, e| e.first().unwrap() - acc)
            })
            .sum()
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        input
            .lines()
            .map(|l| l.trim().split(' ').map(|x| x.parse().unwrap()).collect())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        ";
        assert_eq!(
            Day09Solver::solve_part1(&Day09Solver::parse_input(input.trim())),
            114
        );
    }

    #[test]
    fn test_part2() {
        let input = "
            10 13 16 21 30 45
        ";
        assert_eq!(
            Day09Solver::solve_part2(&Day09Solver::parse_input(input.trim())),
            5
        );
    }
}
