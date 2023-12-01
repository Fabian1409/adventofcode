use std::{collections::BTreeMap, fs::read_to_string};

fn part1() {
    let sum = read_to_string("input_part1.txt")
        .unwrap()
        .lines()
        .fold(0, |acc, l| {
            let digits: Vec<_> = l.chars().filter(|c| c.is_ascii_digit()).collect();
            let first = *digits.first().unwrap() as usize - 0x30;
            let last = *digits.last().unwrap() as usize - 0x30;

            acc + first * 10 + last
        });
    println!("sum = {sum}");
}

fn part2() {
    let digits_str = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let sum = read_to_string("input_part2.txt")
        .unwrap()
        .lines()
        .fold(0, |acc, l| {
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
        });
    println!("sum = {sum}");
}

fn main() {
    part1();
    part2();
}
