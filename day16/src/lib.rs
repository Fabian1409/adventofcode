use std::collections::HashSet;

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offsets(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

fn update_pos(dir: &Direction, row: &mut usize, col: &mut usize) {
    let (row_off, col_off) = dir.offsets();
    *row = (*row as i32 + row_off) as usize;
    *col = (*col as i32 + col_off) as usize;
}

fn bounce(
    grid: &[Vec<char>],
    mut row: usize,
    mut col: usize,
    mut dir: Direction,
    visited: &mut HashSet<(usize, usize, Direction)>,
) {
    while row < grid.len() && col < grid[0].len() {
        if !visited.insert((row, col, dir.clone())) {
            return;
        }

        match grid[row][col] {
            '|' => match dir {
                Direction::Left | Direction::Right => {
                    bounce(grid, row.wrapping_sub(1), col, Direction::Up, visited);
                    bounce(grid, row + 1, col, Direction::Down, visited);
                }
                _ => update_pos(&dir, &mut row, &mut col),
            },
            '-' => match dir {
                Direction::Up | Direction::Down => {
                    bounce(grid, row, col.wrapping_sub(1), Direction::Left, visited);
                    bounce(grid, row, col + 1, Direction::Right, visited);
                }
                _ => update_pos(&dir, &mut row, &mut col),
            },
            '/' => {
                dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
                update_pos(&dir, &mut row, &mut col);
            }
            '\\' => {
                dir = match dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                update_pos(&dir, &mut row, &mut col);
            }
            _ => update_pos(&dir, &mut row, &mut col),
        };
    }
}

fn calc_energy(input: &[Vec<char>], row: usize, col: usize, dir: Direction) -> usize {
    let mut visited = HashSet::new();
    bounce(input, row, col, dir, &mut visited);
    let mut energized = HashSet::new();
    for (row, col, _) in visited {
        energized.insert((row, col));
    }
    energized.len()
}

pub struct Day16Solver;

impl<'a> AdventOfCodeDay<'a> for Day16Solver {
    type ParsedInput = Vec<Vec<char>>;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        calc_energy(input, 0, 0, Direction::Right)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        let mut energy = 0usize;
        for row in 0..input.len() {
            energy = energy.max(calc_energy(input, row, 0, Direction::Right));
            energy = energy.max(calc_energy(input, row, input[0].len() - 1, Direction::Left));
        }

        for col in 0..input[0].len() {
            energy = energy.max(calc_energy(input, 0, col, Direction::Down));
            energy = energy.max(calc_energy(input, input.len() - 1, col, Direction::Up));
        }
        energy
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
        let input = r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        ";
        assert_eq!(
            Day16Solver::solve_part1(&Day16Solver::parse_input(input.trim())),
            46
        );
    }

    #[test]
    fn test_part2() {
        let input = r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        ";
        assert_eq!(
            Day16Solver::solve_part2(&Day16Solver::parse_input(input.trim())),
            51
        );
    }
}
