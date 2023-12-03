#![allow(dead_code)]

use std::collections::HashMap;

fn part1(input: &str) -> usize {
    let schematics: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();
    let mut numbers = HashMap::new();

    for i in 0..schematics.len() {
        for j in 0..schematics[0].len() {
            let c = schematics[i][j];
            if (c != '.') & !c.is_ascii_digit() {
                for i_off in -1..=1i32 {
                    for j_off in -1..=1i32 {
                        let x = if i_off.is_negative() {
                            i.saturating_sub(i_off.unsigned_abs() as usize)
                        } else {
                            (i + i_off as usize).min(schematics.len() - 1)
                        };
                        let y = if j_off.is_negative() {
                            j.saturating_sub(j_off.unsigned_abs() as usize)
                        } else {
                            (j + j_off as usize).min(schematics[0].len() - 1)
                        };
                        let c = schematics[x][y];
                        if c.is_ascii_digit() {
                            let mut digits = String::new();
                            let mut start = y;
                            let mut end = y;
                            for k in (0..y).rev() {
                                if schematics[x][k].is_ascii_digit() {
                                    start -= 1;
                                } else {
                                    break;
                                }
                            }
                            for k in y..schematics[0].len() {
                                if schematics[x][k].is_ascii_digit() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            for k in start..end {
                                digits.push(schematics[x][k])
                            }
                            let number: usize = digits.parse().unwrap();
                            numbers.insert((start..end, x), number);
                        }
                    }
                }
            }
        }
    }
    numbers.values().sum()
}

fn part2(input: &str) -> usize {
    let schematics: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();
    let mut sum = 0;

    for i in 0..schematics.len() {
        for j in 0..schematics[0].len() {
            let c = schematics[i][j];
            if c == '*' {
                let mut numbers = HashMap::new();
                for i_off in -1..=1i32 {
                    for j_off in -1..=1i32 {
                        let x = if i_off.is_negative() {
                            i.saturating_sub(i_off.unsigned_abs() as usize)
                        } else {
                            (i + i_off as usize).min(schematics.len() - 1)
                        };
                        let y = if j_off.is_negative() {
                            j.saturating_sub(j_off.unsigned_abs() as usize)
                        } else {
                            (j + j_off as usize).min(schematics[0].len() - 1)
                        };
                        let c = schematics[x][y];
                        if c.is_ascii_digit() {
                            let mut digits = String::new();
                            let mut start = y;
                            let mut end = y;
                            for k in (0..y).rev() {
                                if schematics[x][k].is_ascii_digit() {
                                    start -= 1;
                                } else {
                                    break;
                                }
                            }
                            for k in y..schematics[0].len() {
                                if schematics[x][k].is_ascii_digit() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            for k in start..end {
                                digits.push(schematics[x][k])
                            }
                            let number: usize = digits.parse().unwrap();
                            numbers.insert((start..end, x), number);
                        }
                    }
                }

                if numbers.len() == 2 {
                    sum += numbers.values().product::<usize>();
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        ";
        assert_eq!(part1(input.trim()), 4361);
    }

    #[test]
    fn test_part2() {
        let input = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        ";
        assert_eq!(part2(input.trim()), 467835);
    }
}
