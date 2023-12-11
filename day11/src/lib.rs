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

fn solve(input: &[Vec<char>], expansion_rate: usize) -> usize {
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

    let mut y_expansions = vec![1usize; input.len()];
    for (i, row) in input.iter().enumerate() {
        if row.iter().all(|c| *c == '.') {
            y_expansions[i] = expansion_rate;
        }
    }

    let mut x_expansions = vec![1usize; input[0].len()];
    for i in 0..input[0].len() {
        if (0..input.len()).map(|j| input[j][i]).all(|c| c == '.') {
            x_expansions[i] = expansion_rate;
        }
    }

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
