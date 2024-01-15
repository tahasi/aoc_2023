use crate::{read_input_file, Result};

mod game;
mod parser;
mod part_one;
mod part_two;

pub fn run_part_one() -> Result<()> {
    let input = read_input_file(10, None)?;
    let steps = part_one::solve(input.trim())?;
    println!("Day ten part one: steps to furthest point: {steps}");
    Ok(())
}

pub fn run_part_two() -> Result<()> {
    let input = read_input_file(10, None)?;
    let count = part_two::solve(input.trim())?;
    println!("Day ten part two: count of enclosed tiles: {count}");
    Ok(())
}
