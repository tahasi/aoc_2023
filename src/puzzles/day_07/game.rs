use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
pub struct CamelCards {
    hands: Vec<Hand>,
}

impl CamelCards {
    pub(crate) fn new(mut hands: Vec<Hand>) -> CamelCards {
        hands.sort();
        CamelCards { hands }
    }

    pub fn winnings(&self) -> usize {
        self.hands
            .iter()
            .enumerate()
            .fold(0, |sum, (rank, hand)| sum + (rank + 1) * hand.bid)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum JacksType {
    Jack,
    Joker,
}

pub(crate) type Cards = [Card; 5];

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Hand {
    bid: usize,
    hand_type: HandType,
    cards: Cards,
    jacks_type: JacksType,
}

impl Hand {
    pub(crate) fn new(bid: usize, cards: Cards, jacks_type: JacksType) -> Hand {
        let hand_type = Hand::cards_hand_type(cards, jacks_type);
        Hand {
            bid,
            hand_type,
            cards,
            jacks_type,
        }
    }

    pub(crate) fn cards_hand_type(cards: Cards, jacks_type: JacksType) -> HandType {
        let grouped = cards
            .iter()
            .fold(HashMap::<Card, usize>::new(), |mut grouped, card| {
                *grouped.entry(*card).or_default() += 1_usize;
                grouped
            });

        match jacks_type {
            JacksType::Jack => Hand::cards_with_jacks_hand_type(grouped),
            JacksType::Joker => Hand::cards_with_jokers_hand_type(grouped),
        }
    }

    fn cards_with_jokers_hand_type(grouped_card_counts: HashMap<Card, usize>) -> HandType {
        use HandType::*;

        if let Some(joker_count) = grouped_card_counts.get(&Card::Jack) {
            let joker_count = *joker_count;
            let mut non_joker_counts = grouped_card_counts
                .iter()
                .filter_map(|(card, count)| {
                    if matches!(card, Card::Jack) {
                        None
                    } else {
                        Some(*count)
                    }
                })
                .collect::<Vec<_>>();
            non_joker_counts.sort();
            match (joker_count, &non_joker_counts[..]) {
                (5, &[]) | (4, &[1]) | (3, &[2]) | (2, &[3]) | (1, &[4]) => FiveOfAKind,
                (3, &[1, 1]) | (2, &[1, 2]) | (1, &[1, 3]) => FourOfAKind,
                (1, &[2, 2]) => FullHouse,
                (2, &[1, 1, 1]) | (1, &[1, 1, 2]) => ThreeOfAKind,
                (1, &[1, 1, 1, 1]) => OnePair,
                _ => panic!("impossible card combo with jokers"),
            }
        } else {
            Hand::cards_with_jacks_hand_type(grouped_card_counts)
        }
    }

    fn cards_with_jacks_hand_type(grouped_card_counts: HashMap<Card, usize>) -> HandType {
        use HandType::*;

        let mut card_counts = grouped_card_counts.values().collect::<Vec<_>>();
        card_counts.sort();
        match card_counts[..] {
            [5] => FiveOfAKind,
            [1, 4] => FourOfAKind,
            [2, 3] => FullHouse,
            [1, 1, 3] => ThreeOfAKind,
            [1, 2, 2] => TwoPair,
            [1, 1, 1, 2] => OnePair,
            [1, 1, 1, 1, 1] => HighCard,
            _ => panic!("impossible card combo with jacks"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                match self
                    .cards
                    .iter()
                    .zip(other.cards)
                    .map(|(a, b)| match self.jacks_type {
                        JacksType::Jack => a.cmp(&b),
                        JacksType::Joker => match (a, b) {
                            (Card::Jack, Card::Jack) => Ordering::Equal,
                            (Card::Jack, _) => Ordering::Less,
                            (_, Card::Jack) => Ordering::Greater,
                            _ => a.cmp(&b),
                        },
                    })
                    .find(|ord| !matches!(ord, Ordering::Equal))
                {
                    None => Ordering::Equal,
                    Some(ord) => ord,
                }
            }
            ord => ord,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash, Ord)]
pub(crate) enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[cfg(test)]
mod tests {
    use super::{CamelCards, Card::*, Hand, HandType::*, JacksType};

    #[test]
    fn winnings_with_jacks() {
        assert_eq!(
            6440,
            CamelCards::new(vec![
                Hand::new(765, [Three, Two, Ten, Three, King], JacksType::Jack),
                Hand::new(684, [Ten, Five, Five, Jack, Five], JacksType::Jack),
                Hand::new(28, [King, King, Six, Seven, Seven], JacksType::Jack),
                Hand::new(220, [King, Ten, Jack, Jack, Ten], JacksType::Jack),
                Hand::new(483, [Queen, Queen, Queen, Jack, Ace], JacksType::Jack),
            ])
            .winnings()
        );
    }

    #[test]
    fn winnings_with_jokers() {
        assert_eq!(
            5905,
            CamelCards::new(vec![
                Hand::new(765, [Three, Two, Ten, Three, King], JacksType::Joker),
                Hand::new(684, [Ten, Five, Five, Jack, Five], JacksType::Joker),
                Hand::new(28, [King, King, Six, Seven, Seven], JacksType::Joker),
                Hand::new(220, [King, Ten, Jack, Jack, Ten], JacksType::Joker),
                Hand::new(483, [Queen, Queen, Queen, Jack, Ace], JacksType::Joker),
            ])
            .winnings()
        );
    }

    #[test]
    fn cards_hand_type_with_jacks() {
        assert_eq!(
            FiveOfAKind,
            Hand::cards_hand_type([Ace, Ace, Ace, Ace, Ace], JacksType::Jack)
        );
        assert_eq!(
            FourOfAKind,
            Hand::cards_hand_type([Ten, Nine, Ten, Ten, Ten], JacksType::Jack)
        );
        assert_eq!(
            FullHouse,
            Hand::cards_hand_type([Nine, Nine, Ten, Ten, Ten], JacksType::Jack)
        );
        assert_eq!(
            ThreeOfAKind,
            Hand::cards_hand_type([Nine, Nine, Nine, Two, Ten], JacksType::Jack)
        );
        assert_eq!(
            TwoPair,
            Hand::cards_hand_type([Nine, Nine, Ten, Two, Ten], JacksType::Jack)
        );
        assert_eq!(
            OnePair,
            Hand::cards_hand_type([Nine, Nine, Ten, Two, Eight], JacksType::Jack)
        );
        assert_eq!(
            OnePair,
            Hand::cards_hand_type([Nine, Nine, Ten, Two, Eight], JacksType::Jack)
        );
        assert_eq!(
            HighCard,
            Hand::cards_hand_type([Nine, King, Ten, Two, Eight], JacksType::Jack)
        );
    }

    #[test]
    fn cards_hand_type_with_jokers() {
        assert_eq!(
            FiveOfAKind,
            Hand::cards_hand_type([Jack, Jack, Jack, Jack, Jack], JacksType::Joker)
        );
        assert_eq!(
            FiveOfAKind,
            Hand::cards_hand_type([Queen, Jack, Jack, Jack, Jack], JacksType::Joker)
        );
        assert_eq!(
            FiveOfAKind,
            Hand::cards_hand_type([Queen, Queen, Jack, Jack, Jack], JacksType::Joker)
        );
        assert_eq!(
            FourOfAKind,
            Hand::cards_hand_type([King, Queen, Jack, Jack, Jack], JacksType::Joker)
        );
        assert_eq!(
            FourOfAKind,
            Hand::cards_hand_type([King, Queen, Queen, Jack, Jack], JacksType::Joker)
        );
        assert_eq!(
            FullHouse,
            Hand::cards_hand_type([Nine, Nine, Ten, Ten, Jack], JacksType::Joker)
        );
        assert_eq!(
            ThreeOfAKind,
            Hand::cards_hand_type([Nine, Nine, Nine, Two, Ten], JacksType::Joker)
        );
        assert_eq!(
            TwoPair,
            Hand::cards_hand_type([Nine, Nine, Ten, Two, Ten], JacksType::Joker)
        );
        assert_eq!(
            OnePair,
            Hand::cards_hand_type([Nine, Nine, Ten, Two, Eight], JacksType::Joker)
        );
        assert_eq!(
            OnePair,
            Hand::cards_hand_type([Nine, Nine, Ten, Two, Eight], JacksType::Joker)
        );
        assert_eq!(
            HighCard,
            Hand::cards_hand_type([Nine, King, Ten, Two, Eight], JacksType::Joker)
        );
    }
}
