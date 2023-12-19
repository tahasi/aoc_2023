use std::ops::Range;

use super::{read_input_file, PuzzleError, Result};

pub fn run_part_one() -> Result<()> {
    let input = read_input_file(6, None)?;
    let product_of_ways_to_win = part_one::solve(input.trim())?;
    println!("Day five part one: product of ways to win: {product_of_ways_to_win}");
    Ok(())
}

pub fn run_part_two() -> Result<()> {
    let input = read_input_file(6, None)?;
    let product_of_ways_to_win = part_two::solve(input.trim())?;
    println!("Day five part two: product of ways to win: {product_of_ways_to_win}");
    Ok(())
}

struct BoatRace {
    time: usize,
    record_distance: usize,
}

impl BoatRace {
    fn winning_charge_times(&self) -> impl Iterator<Item = usize> + '_ {
        (1..self.time - 1)
            .filter(|charge_time| (self.time - charge_time) * charge_time > self.record_distance)
    }

    fn winning_charge_range(&self) -> Range<usize> {
        let mut start = self.time / 2;
        while start > 0 && start * (self.time - start) > self.record_distance {
            start /= 2;
        }
        start = start.max(1);

        while start <= self.time && start * (self.time - start) <= self.record_distance {
            start += 1;
        }

        let mut end = self.time / 2;
        while end <= self.time && end * (self.time - end) > self.record_distance {
            end *= 2;
        }
        end = end.min(self.time - 1);

        while end >= start && end * (self.time - end) <= self.record_distance {
            end -= 1;
        }

        start..end + 1
    }
}

#[cfg(test)]
const INPUT: &str = r"
Time:      7  15   30
Distance:  9  40  200";

mod part_one {
    use super::{
        parser::{parse, NumbersMode},
        Result,
    };

    pub fn solve(input: &str) -> Result<usize> {
        let boat_races = parse(input, NumbersMode::Independent)?;
        let winning_charge_times = boat_races
            .iter()
            .map(|boat_race| boat_race.winning_charge_times());
        let winning_charge_time_counts =
            winning_charge_times.map(|charge_times| charge_times.count());
        let product = winning_charge_time_counts.into_iter().product();
        Ok(product)
    }

    #[cfg(test)]
    mod tests {
        use super::{super::INPUT, solve};

        #[test]
        fn part_one() {
            assert_eq!(Ok(288), solve(INPUT.trim()));
        }
    }
}

mod part_two {
    use super::{
        parser::{parse, NumbersMode},
        Result,
    };

    pub fn solve(input: &str) -> Result<usize> {
        let boat_races = parse(input, NumbersMode::Merge)?;
        let winning_charge_ranges = boat_races
            .iter()
            .map(|boat_race| boat_race.winning_charge_range())
            .collect::<Vec<_>>();
        let winning_charge_time_counts = winning_charge_ranges
            .iter()
            .map(|charge_range| charge_range.end - charge_range.start)
            .collect::<Vec<_>>();
        let product = winning_charge_time_counts.into_iter().product();
        Ok(product)
    }

    #[cfg(test)]
    mod tests {
        use super::{super::INPUT, solve};

        #[test]
        fn part_two() {
            assert_eq!(Ok(71503), solve(INPUT.trim()));
        }
    }
}

mod parser {
    use crate::parser::HasNumbers;

    use super::{BoatRace, PuzzleError, Result};

    pub(super) enum NumbersMode {
        Independent,
        Merge,
    }

    pub(super) fn parse(input: &str, numbers_mode: NumbersMode) -> Result<Vec<BoatRace>> {
        let lines = input
            .trim()
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<_>>();
        if lines.len() != 2 {
            return Err(PuzzleError::invalid_line_input(
                lines.len(),
                "invalid number of lines",
            ));
        }

        if !lines[0].starts_with("Time:") {
            return Err(PuzzleError::invalid_line_input(
                lines.len(),
                "first line must be time",
            ));
        }

        if !lines[1].starts_with("Distance:") {
            return Err(PuzzleError::invalid_line_input(
                lines.len(),
                "second line must be distance",
            ));
        }

        let (times, distances) = match numbers_mode {
            NumbersMode::Independent => {
                (independent_numbers(lines[0]), independent_numbers(lines[1]))
            }
            NumbersMode::Merge => (
                vec![merged_numbers(lines[0])],
                vec![merged_numbers(lines[1])],
            ),
        };

        if times.len() != distances.len() {
            return Err(PuzzleError::invalid_line_input(
                lines.len(),
                "time and distance count of entries must match",
            ));
        }

        Ok(times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| BoatRace {
                time,
                record_distance: distance,
            })
            .collect())
    }

    fn independent_numbers(input: &str) -> Vec<usize> {
        input.numbers().collect::<Vec<_>>()
    }

    fn merged_numbers(input: &str) -> usize {
        input
            .numbers()
            .fold(0, |merged, number| merged * magnitude(number) + number)
    }

    fn magnitude(mut number: usize) -> usize {
        let mut exp = 0;
        while number > 0 {
            number /= 10;
            exp += 1;
        }
        10_usize.pow(exp)
    }
}
