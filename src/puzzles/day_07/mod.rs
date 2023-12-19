use crate::{read_input_file, Result};

mod game;
mod parser;
mod part_one;
mod part_two;

use game::JacksType;

pub fn run_part_one() -> Result<()> {
    let input = read_input_file(7, None)?;
    let winnings = part_one::solve(input.trim(), JacksType::Jack)?;
    println!("Day six part one: camel cards winnings: {winnings}");
    Ok(())
}

pub fn run_part_two() -> Result<()> {
    let input = read_input_file(7, None)?;
    let winnings = part_two::solve(input.trim(), JacksType::Joker)?;
    println!("Day six part two: camel cards with jokers winnings: {winnings}");
    Ok(())
}
