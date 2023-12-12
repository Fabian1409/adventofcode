use aoc_traits::AdventOfCodeDay;

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn solve(input: &[Vec<char>], r: usize) -> usize {
    let galaxies: Vec<_> = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, c)| if *c == '#' { Some((j, i)) } else { None })
                .collect::<Vec<_>>()
        })
        .collect();

    let y_expansions: Vec<_> = input
        .iter()
        .map(|row| if row.iter().any(|c| *c != '.') { 1 } else { r })
        .collect();

    let x_expansions: Vec<_> = (0..input[0].len())
        .map(|i| {
            if (0..input.len()).map(|j| input[j][i]).any(|c| c != '.') {
                1
            } else {
                r
            }
        })
        .collect();

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, galaxy)| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(|other| {
                    let x_adj_galaxy: usize = (0..galaxy.0).map(|x| x_expansions[x]).sum();
                    let y_adj_galaxy: usize = (0..galaxy.1).map(|y| y_expansions[y]).sum();
                    let x_adj_other: usize = (0..other.0).map(|x| x_expansions[x]).sum();
                    let y_adj_other: usize = (0..other.1).map(|y| y_expansions[y]).sum();
                    distance((x_adj_galaxy, y_adj_galaxy), (x_adj_other, y_adj_other))
                })
                .collect::<Vec<_>>()
        })
        .sum()
}

pub struct Day11Solver;

impl<'a> AdventOfCodeDay<'a> for Day11Solver {
    type ParsedInput = Vec<Vec<char>>;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve(input, 2)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve(input, 1_000_000)
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
            82_000_210
        );
    }
}
