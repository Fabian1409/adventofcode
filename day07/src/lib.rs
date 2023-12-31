use std::{cmp::Ordering, str::FromStr};

use aoc_traits::AdventOfCodeDay;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards = cards
            .chars()
            .map(|c| match c {
                'A' => Card::A,
                'K' => Card::K,
                'Q' => Card::Q,
                'J' => Card::J,
                'T' => Card::T,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                _ => Card::Two,
            })
            .collect();

        Ok(Hand {
            cards,
            bid: bid.parse().unwrap(),
        })
    }
}

fn get_type(cards: &[Card]) -> Type {
    let mut counts = [0usize; 13];
    for card in cards {
        counts[card.clone() as usize] += 1;
    }

    let mut counts: Vec<_> = counts.into_iter().filter(|x| *x != 0).collect();
    counts.sort();

    match counts.as_slice() {
        [5] => Type::FiveOfAKind,
        [1, 4] => Type::FourOfAKind,
        [2, 3] => Type::FullHouse,
        [1, 1, 3] => Type::ThreeOfAKind,
        [1, 2, 2] => Type::TwoPair,
        [1, 1, 1, 2] => Type::OnePair,
        _ => Type::HighCard,
    }
}

fn get_new_type(hand_type: Type, num_wildcard: usize) -> Type {
    match (hand_type, num_wildcard) {
        (Type::HighCard, 1) => Type::OnePair,
        (Type::HighCard, 2) => Type::ThreeOfAKind,
        (Type::HighCard, 3) => Type::FourOfAKind,
        (Type::HighCard, 4) => Type::FiveOfAKind,
        (Type::OnePair, 1) => Type::ThreeOfAKind,
        (Type::OnePair, 2) => Type::ThreeOfAKind,
        (Type::OnePair, 3) => Type::FiveOfAKind,
        (Type::TwoPair, 1) => Type::FullHouse,
        (Type::TwoPair, 2) => Type::FourOfAKind,
        (Type::ThreeOfAKind, 1) => Type::FourOfAKind,
        (Type::ThreeOfAKind, 2) => Type::FiveOfAKind,
        (Type::ThreeOfAKind, 3) => Type::FourOfAKind,
        (Type::FullHouse, 2) => Type::FiveOfAKind,
        (Type::FullHouse, 3) => Type::FiveOfAKind,
        (Type::FourOfAKind, 1) => Type::FiveOfAKind,
        (Type::FourOfAKind, 4) => Type::FiveOfAKind,
        (Type::FiveOfAKind, 5) => Type::FiveOfAKind,
        _ => panic!(),
    }
}

pub struct Day07Solver;

impl<'a> AdventOfCodeDay<'a> for Day07Solver {
    type ParsedInput = Vec<Hand>;

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        let mut types: Vec<_> = input
            .iter()
            .map(|hand| (get_type(&hand.cards), hand))
            .collect();
        types.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => a.1.cards.cmp(&b.1.cards),
        });
        types
            .iter()
            .enumerate()
            .map(|(i, (_, hand))| (i + 1) * hand.bid)
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        let mut types: Vec<_> = input
            .iter()
            .map(|hand| {
                let hand_type = get_type(&hand.cards);
                if hand.cards.contains(&Card::J) {
                    let mut counts = [0usize; 13];
                    for card in hand.cards.iter() {
                        counts[card.clone() as usize] += 1;
                    }
                    let num_wildcard = counts[Card::J as usize];
                    let new_type = get_new_type(hand_type, num_wildcard);
                    (new_type, hand)
                } else {
                    (hand_type, hand)
                }
            })
            .collect();
        types.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (a, b) in a.1.cards.iter().zip(&b.1.cards) {
                    if (*a == Card::J) & (*b == Card::J) {
                        continue;
                    } else if *a == Card::J {
                        return Ordering::Less;
                    } else if *b == Card::J {
                        return Ordering::Greater;
                    } else {
                        match a.cmp(b) {
                            Ordering::Equal => continue,
                            x => return x,
                        }
                    }
                }
                Ordering::Equal
            }
        });
        types
            .iter()
            .enumerate()
            .map(|(i, (_, hand))| (i + 1) * hand.bid)
            .sum()
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        input
            .trim()
            .lines()
            .map(|l| l.trim().parse().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        ";
        assert_eq!(
            Day07Solver::solve_part1(&Day07Solver::parse_input(input)),
            6440
        );
    }

    #[test]
    fn test_part2() {
        let input = "
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        ";
        assert_eq!(
            Day07Solver::solve_part2(&Day07Solver::parse_input(input)),
            5905
        );
    }
}
