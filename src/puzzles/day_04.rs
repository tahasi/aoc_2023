use std::collections::HashSet;

use crate::{read_input_file, PuzzleError, Result};

pub fn run_part_one() -> Result<()> {
    let input = read_input_file(4, None)?;
    let sum = part_one::solve(input.trim())?;
    println!("Day four part one: sum of card points: {sum}");
    Ok(())
}

pub fn run_part_two() -> Result<()> {
    let input = read_input_file(4, None)?;
    let sum = part_two::solve(input.trim())?;
    println!("Day four part two: sum of card points: {sum}");
    Ok(())
}

struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl Card {
    fn id(&self) -> u32 {
        self.id
    }

    fn winning_card_count(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count() as u32
    }

    fn score(&self) -> u32 {
        let count = self.winning_card_count();
        if count != 0 {
            2u32.pow(count - 1)
        } else {
            0
        }
    }

    fn winning_card_ids(&self) -> Vec<u32> {
        let count = self.winning_card_count();
        if count == 0 {
            vec![]
        } else {
            let start = self.id + 1;
            let end = self.id + count;
            (start..=end).collect()
        }
    }
}

mod part_one {
    use super::{parser, Result};

    pub fn solve(input: &str) -> Result<u32> {
        let cards = parser::parse(input)?;
        let sum = cards.into_iter().map(|card| card.score()).sum();
        Ok(sum)
    }

    #[cfg(test)]
    mod tests {
        use super::solve;

        #[test]
        fn part_one() {
            let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
            assert_eq!(Ok(13), solve(input.trim()));
        }
    }
}

mod part_two {
    use std::collections::HashMap;

    use super::{parser, Card, Result};

    pub fn solve(input: &str) -> Result<u32> {
        let cards = parser::parse(input)?
            .into_iter()
            .map(|card| (card.id(), card))
            .collect::<HashMap<u32, Card>>();
        let mut sum = cards.len();
        let mut winning_card_ids = cards
            .values()
            .flat_map(|card| card.winning_card_ids())
            .collect::<Vec<u32>>();
        while !winning_card_ids.is_empty() {
            sum += winning_card_ids.len();
            winning_card_ids = winning_card_ids
                .into_iter()
                .flat_map(|card_id| {
                    return cards.get(&card_id).expect("present").winning_card_ids();
                })
                .collect();
        }
        Ok(sum as u32)
    }

    #[cfg(test)]
    mod tests {
        use super::solve;

        #[test]
        fn part_two() {
            let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
            assert_eq!(Ok(30), solve(input.trim()));
        }
    }
}

mod parser {
    use std::collections::HashSet;

    use super::{Card, PuzzleError, Result};
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref RE_CARD_TOKEN: Regex =
            Regex::new(r"(\d+|:|\|)").expect("failed card token regex");
    }

    pub fn parse(input: &str) -> Result<Vec<Card>> {
        input
            .lines()
            .enumerate()
            .map(|(line, input)| {
                let builder = RE_CARD_TOKEN.captures_iter(input).try_fold(
                    CardBuilder::new(),
                    |builder, capture| {
                        if let Some(token) = capture.get(1) {
                            builder.process_token(token.as_str())
                        } else {
                            Err(PuzzleError::invalid_line_input(line, "failed to parse"))
                        }
                    },
                );
                builder?.build()
            })
            .collect()
    }

    enum CardBuilderState {
        GameId,
        WinningNumbers,
        Numbers,
    }

    struct CardBuilder {
        state: CardBuilderState,
        id: Option<u32>,
        winning_numbers: HashSet<u32>,
        numbers: HashSet<u32>,
    }

    impl CardBuilder {
        fn new() -> Self {
            CardBuilder {
                state: CardBuilderState::GameId,
                id: None,
                winning_numbers: HashSet::new(),
                numbers: HashSet::new(),
            }
        }

        fn process_token(mut self, token: &str) -> Result<Self> {
            if let Ok(number) = token.parse::<u32>() {
                match self.state {
                    CardBuilderState::GameId => {
                        if let Some(id) = self.id {
                            return Err(PuzzleError::unexpected(&format!(
                                "already received game id: {id}"
                            )));
                        }
                        self.id = Some(number);
                    }
                    CardBuilderState::WinningNumbers => {
                        self.winning_numbers.insert(number);
                    }
                    CardBuilderState::Numbers => {
                        self.numbers.insert(number);
                    }
                }
            } else {
                match token {
                    ":" => self.state = CardBuilderState::WinningNumbers,
                    "|" => self.state = CardBuilderState::Numbers,
                    _ => return Err(PuzzleError::unexpected("invalid token: {token}")),
                }
            }
            Ok(self)
        }

        fn build(self) -> Result<Card> {
            match (self.state, self.id) {
                (CardBuilderState::Numbers, Some(id)) => Ok(Card {
                    id,
                    winning_numbers: self.winning_numbers,
                    numbers: self.numbers,
                }),
                _ => Err(PuzzleError::unexpected("invalid parse")),
            }
        }
    }
}
