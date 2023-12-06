use aoc_traits::AdventOfCodeDay;

pub struct Day01Solver;

impl<'a> AdventOfCodeDay<'a> for Day01Solver {
    type ParsedInput = &'a str;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        input.lines().fold(0, |acc, l| {
            let digits: Vec<_> = l.chars().filter(|c| c.is_ascii_digit()).collect();
            let first = *digits.first().unwrap() as usize - 0x30;
            let last = *digits.last().unwrap() as usize - 0x30;

            acc + first * 10 + last
        })
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        input.lines().fold(0, |acc, l| {
            let digits: Vec<usize> = l
                .chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    if c.is_ascii_digit() {
                        Some(c as usize - 0x30)
                    } else {
                        match &l[i..] {
                            "one" => Some(1),
                            "two" => Some(2),
                            "three" => Some(3),
                            "four" => Some(4),
                            "five" => Some(5),
                            "six" => Some(6),
                            "seven" => Some(7),
                            "eight" => Some(8),
                            "nine" => Some(9),
                            _ => None,
                        }
                    }
                })
                .collect();

            acc + digits.first().unwrap() * 10 + digits.last().unwrap()
        })
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        input
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        ";
        assert_eq!(
            Day01Solver::solve_part2(&Day01Solver::parse_input(input.trim())),
            142
        );
    }

    #[test]
    fn test_part2() {
        let input = "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        ";
        assert_eq!(
            Day01Solver::solve_part2(&Day01Solver::parse_input(input.trim())),
            281
        );
    }
}
