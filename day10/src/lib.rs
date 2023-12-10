use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn find_start_dir(input: &[Vec<char>], x: &mut usize, y: &mut usize) -> Option<Direction> {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0i32, -1i32)];
    for (x_off, y_off) in dirs {
        let cur_x = *x as i32 + x_off;
        let cur_y = *y as i32 + y_off;
        if let Some(row) = input.get(cur_y as usize) {
            if let Some(c) = row.get(cur_x as usize) {
                if (x_off == -1) & next_dir(Direction::West, *c).is_some() {
                    *x = cur_x as usize;
                    *y = cur_y as usize;
                    return Some(Direction::West);
                } else if (x_off == 1) & next_dir(Direction::East, *c).is_some() {
                    *x = cur_x as usize;
                    *y = cur_y as usize;
                    return Some(Direction::East);
                } else if (y_off == -1) & next_dir(Direction::North, *c).is_some() {
                    *x = cur_x as usize;
                    *y = cur_y as usize;
                    return Some(Direction::North);
                } else if (y_off == 1) & next_dir(Direction::South, *c).is_some() {
                    *x = cur_x as usize;
                    *y = cur_y as usize;
                    return Some(Direction::South);
                } else {
                    continue;
                }
            }
        }
    }
    None
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

fn fill_row(data: &mut [Vec<char>], row: usize, col: usize, clockwise: bool) -> usize {
    let mut num = 0;
    let width = data[0].len();
    if !clockwise {
        for i in (0..col).rev() {
            if data[row][i] == '*' {
                break;
            }
            data[row][i] = 'I';
            num += 1;
        }
    } else {
        for i in col..width {
            if data[row][i] == '*' {
                break;
            }
            data[row][i] = 'I';
            num += 1;
        }
    }
    num
}

pub struct Day10Solver;

impl<'a> AdventOfCodeDay<'a> for Day10Solver {
    type ParsedInput = Vec<Vec<char>>;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        let mut y = input.iter().position(|row| row.contains(&'S')).unwrap();
        let mut x = input[y].iter().position(|c| *c == 'S').unwrap();
        let mut dir = find_start_dir(input, &mut x, &mut y).unwrap();
        let mut len = 1;

        while input[y][x] != 'S' {
            dir = next_dir(dir, input[y][x]).unwrap();
            len += 1;
            match dir {
                Direction::North => y -= 1,
                Direction::South => y += 1,
                Direction::East => x += 1,
                Direction::West => x -= 1,
            }
        }

        len / 2
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        let mut data = input.clone();
        let mut y = input.iter().position(|row| row.contains(&'S')).unwrap();
        let mut x = input[y].iter().position(|c| *c == 'S').unwrap();
        let mut loop_pos = Vec::new();
        data[y][x] = '*';

        let mut dir = find_start_dir(input, &mut x, &mut y).unwrap();
        let first_dir = dir.clone();

        let mut num_left_turns = 0;
        let mut num_right_turns = 0;

        while input[y][x] != 'S' {
            loop_pos.push((x, y));
            let new_dir = next_dir(dir.clone(), input[y][x]).unwrap();
            data[y][x] = '*';
            match new_dir {
                Direction::North => y -= 1,
                Direction::South => y += 1,
                Direction::East => x += 1,
                Direction::West => x -= 1,
            }

            match (dir, new_dir.clone()) {
                (Direction::North, Direction::East) => num_right_turns += 1,
                (Direction::North, Direction::West) => num_left_turns += 1,
                (Direction::South, Direction::East) => num_left_turns += 1,
                (Direction::South, Direction::West) => num_right_turns += 1,
                (Direction::East, Direction::North) => num_left_turns += 1,
                (Direction::East, Direction::South) => num_right_turns += 1,
                (Direction::West, Direction::North) => num_right_turns += 1,
                (Direction::West, Direction::South) => num_left_turns += 1,
                _ => {}
            }

            dir = new_dir;
        }

        // for row in data.iter() {
        //     println!("{row:?}");
        // }

        // println!("{loop_pos:?}");

        let mut num = 0;
        let clockwise = num_right_turns < num_left_turns;

        let mut dir = first_dir;
        for (x, y) in loop_pos {
            match dir {
                Direction::North => num += fill_row(&mut data, y, x, !clockwise),
                Direction::South => num += fill_row(&mut data, y, x, clockwise),
                _ => match next_dir(dir.clone(), input[y][x]).unwrap() {
                    Direction::North => num += fill_row(&mut data, y, x, !clockwise),
                    Direction::South => num += fill_row(&mut data, y, x, clockwise),
                    _ => {}
                },
            }
            dir = next_dir(dir, input[y][x]).unwrap();
        }

        // for row in data.iter() {
        //     println!("{row:?}");
        // }

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
