use super::{game::JacksType, parser::parse};
use crate::Result;

pub fn solve(input: &str, jacks_type: JacksType) -> Result<usize> {
    let camel_cards = parse(input, jacks_type)?;
    Ok(camel_cards.winnings())
}

#[cfg(test)]
mod tests {
    use super::{solve, JacksType};

    const INPUT: &str = r"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_one() {
        assert_eq!(Ok(6440), solve(INPUT, JacksType::Jack));
    }
}
