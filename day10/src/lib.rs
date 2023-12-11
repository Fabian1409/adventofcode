use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn find_start_dir(input: &[Vec<char>], col: usize, row: usize) -> Option<Direction> {
    if (col != 0) & next_dir(Direction::West, input[row][col - 1]).is_some() {
        Some(Direction::West)
    } else if (col != input[0].len()) & next_dir(Direction::East, input[row][col + 1]).is_some() {
        Some(Direction::East)
    } else if (row != 0) & next_dir(Direction::North, input[row - 1][col]).is_some() {
        Some(Direction::North)
    } else if (row != input.len()) & next_dir(Direction::South, input[row + 1][col]).is_some() {
        Some(Direction::South)
    } else {
        None
    }
}

fn next_dir(dir: Direction, c: char) -> Option<Direction> {
    match dir {
        Direction::North => match c {
            '|' => Some(Direction::North),
            'F' => Some(Direction::East),
            '7' => Some(Direction::West),
            _ => None,
        },
        Direction::South => match c {
            '|' => Some(Direction::South),
            'L' => Some(Direction::East),
            'J' => Some(Direction::West),
            _ => None,
        },
        Direction::East => match c {
            '-' => Some(Direction::East),
            'J' => Some(Direction::North),
            '7' => Some(Direction::South),
            _ => None,
        },
        Direction::West => match c {
            '-' => Some(Direction::West),
            'L' => Some(Direction::North),
            'F' => Some(Direction::South),
            _ => None,
        },
    }
}

fn fill_row(data: &[Vec<char>], row: usize, col: usize, clockwise: bool) -> usize {
    let mut num = 0;
    let width = data[0].len();
    if !clockwise {
        for i in (0..col).rev() {
            if data[row][i] == '*' {
                break;
            }
            num += 1;
        }
    } else {
        for i in col..width {
            if data[row][i] == '*' {
                break;
            }
            num += 1;
        }
    }
    num
}

fn move_in_dir(dir: &Direction, col: &mut usize, row: &mut usize) {
    match dir {
        Direction::North => *row -= 1,
        Direction::South => *row += 1,
        Direction::East => *col += 1,
        Direction::West => *col -= 1,
    }
}

pub struct Day10Solver;

impl<'a> AdventOfCodeDay<'a> for Day10Solver {
    type ParsedInput = Vec<Vec<char>>;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        let mut row = input.iter().position(|row| row.contains(&'S')).unwrap();
        let mut col = input[row].iter().position(|c| *c == 'S').unwrap();
        let mut dir = find_start_dir(input, col, row).unwrap();
        move_in_dir(&dir, &mut col, &mut row);

        let mut len = 1;
        while input[row][col] != 'S' {
            dir = next_dir(dir, input[row][col]).unwrap();
            move_in_dir(&dir, &mut col, &mut row);
            len += 1;
        }

        len / 2
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        let mut data = input.clone();
        let mut row = input.iter().position(|row| row.contains(&'S')).unwrap();
        let mut col = input[row].iter().position(|c| *c == 'S').unwrap();
        let mut loop_pos = Vec::new();

        data[row][col] = '*';

        let mut dir = find_start_dir(input, col, row).unwrap();
        let start_dir = dir.clone();
        move_in_dir(&dir, &mut col, &mut row);

        let mut left_turns = 0;
        let mut right_turns = 0;

        while input[row][col] != 'S' {
            loop_pos.push((col, row));
            data[row][col] = '*';

            let new_dir = next_dir(dir.clone(), input[row][col]).unwrap();
            move_in_dir(&new_dir, &mut col, &mut row);

            match (dir, new_dir.clone()) {
                (Direction::North, Direction::East) => right_turns += 1,
                (Direction::North, Direction::West) => left_turns += 1,
                (Direction::South, Direction::East) => left_turns += 1,
                (Direction::South, Direction::West) => right_turns += 1,
                (Direction::East, Direction::North) => left_turns += 1,
                (Direction::East, Direction::South) => right_turns += 1,
                (Direction::West, Direction::North) => right_turns += 1,
                (Direction::West, Direction::South) => left_turns += 1,
                _ => {}
            }

            dir = new_dir;
        }

        let mut num = 0;
        let clockwise = right_turns < left_turns;
        let mut dir = start_dir;

        for (col, row) in loop_pos {
            match dir {
                Direction::North => num += fill_row(&data, row, col, !clockwise),
                Direction::South => num += fill_row(&data, row, col, clockwise),
                _ => match next_dir(dir.clone(), input[row][col]).unwrap() {
                    Direction::North => num += fill_row(&data, row, col, !clockwise),
                    Direction::South => num += fill_row(&data, row, col, clockwise),
                    _ => {}
                },
            }
            dir = next_dir(dir, input[row][col]).unwrap();
        }
        num
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
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        ";
        assert_eq!(
            Day10Solver::solve_part1(&Day10Solver::parse_input(input.trim())),
            4
        );
    }

    #[test]
    fn test_part2() {
        let input = "
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        ";
        assert_eq!(
            Day10Solver::solve_part2(&Day10Solver::parse_input(input.trim())),
            8
        );
    }
}
