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

pub struct Day14Solver;

impl<'a> AdventOfCodeDay<'a> for Day14Solver {
    type ParsedInput = Vec<Vec<char>>;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        input
            .iter()
            .map(|row| {
                let mut round = vec![(0, 0)];
                for (i, c) in row.iter().enumerate() {
                    match c {
                        'O' => round.last_mut().unwrap().1 += 1,
                        '#' => round.push((i + 1, 0)),
                        _ => {}
                    }
                }
                round
                    .into_iter()
                    .map(|(pos, num)| (pos..pos + num).map(|x| row.len() - x).sum::<usize>())
                    .sum::<usize>()
            })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        todo!()
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        transpose(input.lines().map(|l| l.trim().chars().collect()).collect())
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
            0
        );
    }
}
