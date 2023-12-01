#![allow(dead_code)]

use std::collections::BTreeMap;

fn part1(input: &str) -> usize {
    input.lines().fold(0, |acc, l| {
        let digits: Vec<_> = l.chars().filter(|c| c.is_ascii_digit()).collect();
        let first = *digits.first().unwrap() as usize - 0x30;
        let last = *digits.last().unwrap() as usize - 0x30;

        acc + first * 10 + last
    })
}

fn part2(input: &str) -> usize {
    let digits_str = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input.lines().fold(0, |acc, l| {
        let mut digit_pos = BTreeMap::new();
        for (digit, digit_str) in digits_str.iter().enumerate() {
            if let Some(pos) = l.find(digit.to_string().as_str()) {
                digit_pos.insert(pos, digit);
            }
            if let Some(pos) = l.rfind(digit.to_string().as_str()) {
                digit_pos.insert(pos, digit);
            }
            if let Some(pos) = l.find(digit_str) {
                digit_pos.insert(pos, digit);
            }
            if let Some(pos) = l.rfind(digit_str) {
                digit_pos.insert(pos, digit);
            }
        }
        let first = digit_pos.values().next().unwrap_or(&0);
        let last = digit_pos.values().last().unwrap_or(&0);

        acc + first * 10 + last
    })
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
        assert_eq!(part1(input.trim()), 142)
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
        assert_eq!(part2(input.trim()), 281)
    }
}
