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
        let digits_int = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let digits_str = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        input.lines().fold(0, |acc, l| {
            let first = digits_int
                .iter()
                .zip(digits_str)
                .enumerate()
                .map(|(i, (int, str))| {
                    (
                        i,
                        l.find(int)
                            .unwrap_or(usize::MAX)
                            .min(l.find(str).unwrap_or(usize::MAX)),
                    )
                })
                .min_by_key(|(_, pos)| *pos)
                .unwrap()
                .0;
            let last = digits_int
                .iter()
                .zip(digits_str)
                .enumerate()
                .map(|(i, (int, str))| {
                    (i, l.rfind(int).unwrap_or(0).max(l.rfind(str).unwrap_or(0)))
                })
                .rev()
                .max_by_key(|(_, pos)| *pos)
                .unwrap()
                .0;

            acc + first * 10 + if last != 0 { last } else { first }
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
