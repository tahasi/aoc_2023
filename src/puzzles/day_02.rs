use crate::{read_input_file, Result};

pub fn run_part_one() -> Result<()> {
    let input = read_input_file(2, None)?;
    let bag_content = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    let sum = part_one::solve(input.trim(), &bag_content)?;
    println!("Day two part one: sum of possible game identifiers: {sum}");
    Ok(())
}

pub fn run_part_two() -> Result<()> {
    let input = read_input_file(2, None)?;
    let sum = part_two::solve(input.trim())?;
    println!("Day two part two: sum of required bag set powers: {sum}");
    Ok(())
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        Set { red, green, blue }
    }

    fn empty() -> Self {
        Self::new(0, 0, 0)
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }

    fn contains(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn union(&self, other: &Self) -> Self {
        Self::new(
            usize::max(self.red, other.red),
            usize::max(self.green, other.green),
            usize::max(self.blue, other.blue),
        )
    }
}

mod part_one {
    use super::{parser, Set};
    use crate::Result;
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref RE_GAME_ID: Regex = Regex::new(r"Game (\d+)").expect("failed game id regex");
        static ref RE_SETS: Regex =
            Regex::new(r"(\d+) (red|green|blue),?").expect("failed sets regex");
    }

    pub fn solve(input: &str, bag_content: &Set) -> Result<usize> {
        let game_sets = parser::parse(input)?
            .into_iter()
            .filter(|(_, sets)| sets.iter().all(|set| bag_content.contains(set)));
        let sum = game_sets.fold(0, |sum, (game_id, _sets)| sum + game_id);

        Ok(sum)
    }

    #[cfg(test)]
    mod tests {
        use super::{solve, Set};

        #[test]
        fn part_one() {
            let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
            let bag_content = Set {
                red: 12,
                green: 13,
                blue: 14,
            };
            assert_eq!(Ok(8), solve(input.trim(), &bag_content));
        }
    }
}

mod part_two {
    use super::{parser, Result, Set};

    pub fn solve(input: &str) -> Result<u32> {
        Ok(parser::parse(input)?
            .into_iter()
            .fold(0usize, |sum, (_, sets)| {
                let super_set = sets
                    .into_iter()
                    .fold(Set::empty(), |super_set, set| super_set.union(&set));
                sum + super_set.power()
            }) as u32)
    }

    #[cfg(test)]
    mod tests {
        use super::solve;

        #[test]
        fn test_two() {
            let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
            assert_eq!(Ok(2286), solve(input.trim()));
        }
    }
}

mod parser {
    use super::Set;
    use crate::{PuzzleError, Result};
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref RE_GAME_ID: Regex = Regex::new(r"Game (\d+)").expect("failed game id regex");
        static ref RE_SETS: Regex =
            Regex::new(r"(\d+) (red|green|blue),?").expect("failed sets regex");
    }

    impl Set {
        fn builder() -> SetBuilder {
            SetBuilder::new()
        }
    }

    struct SetBuilder {
        red: usize,
        green: usize,
        blue: usize,
    }

    impl SetBuilder {
        fn new() -> Self {
            SetBuilder {
                red: 0,
                green: 0,
                blue: 0,
            }
        }

        fn set_red(&mut self, count: usize) {
            self.red = count;
        }

        fn set_green(&mut self, count: usize) {
            self.green = count;
        }

        fn set_blue(&mut self, count: usize) {
            self.blue = count;
        }

        fn build(self) -> Set {
            Set::new(self.red, self.green, self.blue)
        }
    }

    pub fn parse(input: &str) -> Result<Vec<(usize, Vec<Set>)>> {
        input
            .lines()
            .enumerate()
            .map(|(line, input)| {
                let split_input: Vec<&str> = input.split(':').collect();
                if split_input.len() != 2 {
                    return Err(PuzzleError::invalid_line_input(
                        line,
                        "incorrect number of semi colons",
                    ));
                }
                let id = parse_game_id(split_input[0]).ok_or_else(|| {
                    PuzzleError::invalid_line_input(line, "missing game identifier")
                })?;
                let sets = split_input[1]
                    .split(';')
                    .map(|set| {
                        parse_game_set(set).ok_or_else(|| {
                            PuzzleError::invalid_line_input(line, "invalid set '{set}'")
                        })
                    })
                    .collect::<Result<Vec<Set>>>()?;
                if sets.is_empty() {
                    return Err(PuzzleError::invalid_line_input(line, "missing games sets"));
                }
                Ok((id, sets))
            })
            .collect()
    }

    fn parse_game_id(input: &str) -> Option<usize> {
        RE_GAME_ID.captures(input).and_then(|capture| {
            capture
                .get(1)
                .and_then(|sub_capture| sub_capture.as_str().parse::<usize>().ok())
        })
    }

    fn parse_game_set(input: &str) -> Option<Set> {
        Some(
            RE_SETS
                .captures_iter(input)
                .try_fold(Set::builder(), |mut builder, capture| {
                    let count = capture
                        .get(1)
                        .and_then(|sub_capture| sub_capture.as_str().parse::<usize>().ok());
                    let color = capture.get(2).map(|sub_capture| sub_capture.as_str());
                    match (count, color) {
                        (Some(count), Some(color)) => match color {
                            "red" => {
                                builder.set_red(count);
                                Some(builder)
                            }
                            "green" => {
                                builder.set_green(count);
                                Some(builder)
                            }
                            "blue" => {
                                builder.set_blue(count);
                                Some(builder)
                            }
                            _ => None,
                        },
                        _ => None,
                    }
                })?
                .build(),
        )
    }
}
