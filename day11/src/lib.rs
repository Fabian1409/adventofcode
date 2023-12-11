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
        let mut expanded = Vec::new();
        for row in input.iter() {
            expanded.push(row.clone());
            if row.iter().all(|c| *c == '.') {
                expanded.push(row.clone());
            }
        }

        let transposed = transpose(expanded);

        let mut expanded = Vec::new();

        for row in transposed.iter() {
            expanded.push(row.clone());
            if row.iter().all(|c| *c == '.') {
                expanded.push(row.clone());
            }
        }

        let expanded = transpose(expanded);

        let galaxies: Vec<_> = expanded
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
                    .map(|other| distance(*galaxy, *other))
                    .collect::<Vec<_>>()
            })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        todo!()
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
            8410
        );
    }
}
