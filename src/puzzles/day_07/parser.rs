use crate::{result::PuzzleError, Result};

use super::game::{CamelCards, Card, Cards, Hand, JacksType};

pub(super) fn parse(input: &str, jacks_type: JacksType) -> Result<CamelCards> {
    let hands = input
        .lines()
        .enumerate()
        .filter_map(|(line, input)| {
            let input = input.trim();
            if input.is_empty() {
                None
            } else {
                Some((line, input))
            }
        })
        .map(|(line, input)| {
            let cards_bid = input
                .trim()
                .split(' ')
                .map(|segment| segment.trim())
                .collect::<Vec<_>>();
            if cards_bid.len() != 2
                || cards_bid[0].len() != 5
                || cards_bid[1].chars().any(|c| !c.is_ascii_digit())
            {
                return Err(PuzzleError::invalid_line_input(
                    line,
                    "invalid cards bid format",
                ));
            }

            let cards = cards_bid[0]
                .chars()
                .map(Card::try_from)
                .collect::<Result<Vec<_>>>()?;
            let bid = cards_bid[1]
                .parse::<usize>()
                .map_err(|_err| PuzzleError::invalid_line_input(line, "failed to parse bid"))?;
            Hand::try_new(bid, &cards, jacks_type)
        })
        .collect::<Result<Vec<Hand>>>()?;
    Ok(CamelCards::new(hands))
}

impl Hand {
    fn try_new(bid: usize, cards: &[Card], jacks_type: JacksType) -> Result<Hand> {
        match cards.len() {
            5 => {
                let cards: Cards = cards
                    .try_into()
                    .expect("failed to create cards from vector");
                Ok(Hand::new(bid, cards, jacks_type))
            }
            _ => Err(PuzzleError::invalid_input(&format!(
                "invalid count of cards in vector {}",
                cards.len()
            ))),
        }
    }
}

impl TryFrom<char> for Card {
    type Error = PuzzleError;

    fn try_from(value: char) -> Result<Self> {
        use Card::*;

        match value {
            'A' => Ok(Ace),
            'K' => Ok(King),
            'Q' => Ok(Queen),
            'J' => Ok(Jack),
            'T' => Ok(Ten),
            '9' => Ok(Nine),
            '8' => Ok(Eight),
            '7' => Ok(Seven),
            '6' => Ok(Six),
            '5' => Ok(Five),
            '4' => Ok(Four),
            '3' => Ok(Three),
            '2' => Ok(Two),
            _ => Err(PuzzleError::invalid_input(&format!(
                "invalid card '{}'",
                value
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        parse, CamelCards,
        Card::{self, *},
        Hand, JacksType,
    };
    use crate::{result::PuzzleError, Result};

    const INPUT: &str = r"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn parse_input() {
        let expected_camel_cards = CamelCards::new(vec![
            Hand::new(765, [Three, Two, Ten, Three, King], JacksType::Jack),
            Hand::new(684, [Ten, Five, Five, Jack, Five], JacksType::Jack),
            Hand::new(28, [King, King, Six, Seven, Seven], JacksType::Jack),
            Hand::new(220, [King, Ten, Jack, Jack, Ten], JacksType::Jack),
            Hand::new(483, [Queen, Queen, Queen, Jack, Ace], JacksType::Jack),
        ]);
        assert_eq!(
            Ok(expected_camel_cards),
            parse(INPUT.trim(), JacksType::Jack)
        );
    }

    #[test]
    fn card_try_from_char() {
        assert_eq!(
            Ok(vec![Ace, King, Queen]),
            "AKQ"
                .chars()
                .map(|c| Card::try_from(c))
                .collect::<Result<Vec<_>>>()
        );

        assert_eq!(
            Ok(vec![Ten, Nine, Eight, Seven, Two, Three, Four, Five, Six]),
            "T98723456"
                .chars()
                .map(|c| Card::try_from(c))
                .collect::<Result<Vec<_>>>()
        );

        assert_eq!(
            Err(PuzzleError::invalid_input("invalid card '1'")),
            "T9817"
                .chars()
                .map(|c| Card::try_from(c))
                .collect::<Result<Vec<_>>>()
        );

        assert_eq!(
            Err(PuzzleError::invalid_input("invalid card 'Z'")),
            "T98Z27"
                .chars()
                .map(|c| Card::try_from(c))
                .collect::<Result<Vec<_>>>()
        );
    }

    #[test]
    fn card_order() {
        assert!(
            Ace > King
                && King > Queen
                && Queen > Jack
                && Jack > Ten
                && Ten > Nine
                && Nine > Eight
                && Eight > Seven
                && Seven > Six
                && Six > Five
                && Five > Four
                && Four > Three
                && Three > Two
        );
    }
}
