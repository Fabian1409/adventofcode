#![allow(dead_code)]

use std::collections::HashMap;

fn part1(input: &str) -> usize {
    let schematics: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();
    let mut parts = HashMap::new();

    for (i, line) in schematics.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if (*c != '.') & !c.is_ascii_digit() {
                for i_off in 0..=2 {
                    for j_off in 0..=2 {
                        let x = (i + i_off).saturating_sub(1).min(schematics.len() - 1);
                        let y = (j + j_off).saturating_sub(1).min(line.len() - 1);
                        let c = schematics[x][y];
                        if c.is_ascii_digit() {
                            let mut digits = String::new();
                            let mut start = y;
                            let mut end = y;
                            for c in schematics[x].iter().take(y).rev() {
                                if c.is_ascii_digit() {
                                    start -= 1;
                                } else {
                                    break;
                                }
                            }
                            for c in schematics[x].iter().skip(y) {
                                if c.is_ascii_digit() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            for c in schematics[x].iter().skip(start).take(end - start) {
                                digits.push(*c)
                            }
                            let number: usize = digits.parse().unwrap();
                            parts.insert((start..end, x), number);
                        }
                    }
                }
            }
        }
    }
    parts.values().sum()
}

fn part2(input: &str) -> usize {
    let schematics: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();
    let mut sum = 0;

    for (i, line) in schematics.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '*' {
                let mut gears = HashMap::new();
                for i_off in 0..=2 {
                    for j_off in 0..=2 {
                        let x = (i + i_off).saturating_sub(1).min(schematics.len() - 1);
                        let y = (j + j_off).saturating_sub(1).min(schematics[0].len() - 1);
                        let c = schematics[x][y];
                        if c.is_ascii_digit() {
                            let mut digits = String::new();
                            let mut start = y;
                            let mut end = y;
                            for c in schematics[x].iter().take(y).rev() {
                                if c.is_ascii_digit() {
                                    start -= 1;
                                } else {
                                    break;
                                }
                            }
                            for c in schematics[x].iter().skip(y) {
                                if c.is_ascii_digit() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            for c in schematics[x].iter().skip(start).take(end - start) {
                                digits.push(*c)
                            }
                            let number: usize = digits.parse().unwrap();
                            gears.insert((start..end, x), number);
                        }
                    }
                }

                if gears.len() == 2 {
                    sum += gears.values().product::<usize>();
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
