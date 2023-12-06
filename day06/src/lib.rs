use aoc_traits::AdventOfCodeDay;

#[derive(Debug)]
pub struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn new(time: usize, record: usize) -> Self {
        Race { time, record }
    }
}

pub struct Day06Solver;

impl<'a> AdventOfCodeDay<'a> for Day06Solver {
    type ParsedInput = (Vec<Race>, Race);

    type Part1Output = usize;

    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        input
            .0
            .iter()
            .map(|race| {
                let b = race.time as f32;
                let c = -(race.record as f32);
                let tmp = (b.powf(2f32) + 4f32 * c).sqrt();
                let x0 = (b - tmp) * 0.5;
                let x1 = (b + tmp) * 0.5;
                ((x1.ceil() - 1f32) - x0.floor()) as usize
            })
            .product()
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        let race = &input.1;
        let b = race.time as f64;
        let c = -(race.record as f64);
        let tmp = (b.powf(2f64) + 4f64 * c).sqrt();
        let x0 = (b - tmp) * 0.5;
        let x1 = (b + tmp) * 0.5;
        ((x1.ceil() - 1f64) - x0.floor()) as usize
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        let (times, distances) = input.trim().split_once('\n').unwrap();
        let times: Vec<_> = times
            .trim()
            .strip_prefix("Time:")
            .unwrap()
            .split_whitespace()
            .collect();
        let distances: Vec<_> = distances
            .trim()
            .strip_prefix("Distance:")
            .unwrap()
            .split_whitespace()
            .collect();

        let race = Race::new(
            times.join("").parse().unwrap(),
            distances.join("").parse().unwrap(),
        );

        let races = times
            .iter()
            .zip(distances)
            .map(|(t, d)| Race::new(t.parse().unwrap(), d.parse().unwrap()))
            .collect();

        (races, race)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
            Time:      7  15   30
            Distance:  9  40  200
        ";
        assert_eq!(
            Day06Solver::solve_part1(&Day06Solver::parse_input(input)),
            288
        );
    }

    #[test]
    fn test_part2() {
        let input = "
            Time:      7  15   30
            Distance:  9  40  200
        ";
        assert_eq!(
            Day06Solver::solve_part2(&Day06Solver::parse_input(input)),
            71503
        );
    }
}
