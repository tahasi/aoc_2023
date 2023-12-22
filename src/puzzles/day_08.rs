use std::{collections::HashMap, fmt::Display};

use crate::{read_input_file, PuzzleError, Result};

pub fn run_part_one() -> Result<()> {
    let input = read_input_file(8, None)?;
    let steps = part_one::solve(input.trim())?;
    println!("Day eight part one: steps from AAA to ZZZ: {steps}");
    Ok(())
}

pub fn run_part_two() -> Result<()> {
    let input = read_input_file(8, None)?;
    let steps = part_two::solve(input.trim())?;
    println!("Day eight part one: ghost steps from ..A to ..Z: {steps}");
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GuidedMap {
    directions: Directions,
    network: HashMap<String, (String, String)>,
}

impl GuidedMap {
    fn new(directions: Vec<Direction>, network: HashMap<String, (String, String)>) -> GuidedMap {
        let directions = Directions::new(directions);
        GuidedMap {
            directions,
            network,
        }
    }

    fn steps_count(&mut self) -> usize {
        let mut count = 0;
        let mut node = "AAA";
        while node != "ZZZ" {
            let (left, right) = self
                .network
                .get(node)
                .unwrap_or_else(|| panic!("node '{node}' not found in the map"));
            count += 1;
            node = match self.directions.next() {
                Direction::Left => left,
                Direction::Right => right,
            };
        }
        count
    }

    fn ghost_steps_count(&mut self) -> usize {
        let nodes = self
            .network
            .keys()
            .filter(|node| node.ends_with('A'))
            .collect::<Vec<_>>();
        let mut steps_to_z_count = nodes
            .iter()
            .map(|start_node| {
                self.directions.reset();
                let mut count = 0;
                let mut node: &str = start_node.as_ref();
                while !node.ends_with('Z') {
                    count += 1;
                    let (left, right) = self.network.get(node).expect("node in map");
                    node = match self.directions.next() {
                        Direction::Left => left,
                        Direction::Right => right,
                    };
                }
                count
            })
            .collect::<Vec<_>>();
        steps_to_z_count.sort();
        let gcd = steps_to_z_count.iter().fold(0, |gcd, count| {
            if gcd != 0 {
                num::integer::gcd(gcd, *count)
            } else {
                *count
            }
        });
        let multiples_of_gcd = steps_to_z_count
            .iter()
            .map(|count| *count / gcd)
            .collect::<Vec<_>>();
        multiples_of_gcd.iter().product::<usize>() * gcd
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Directions {
    directions: Vec<Direction>,
    direction_next: usize,
    iteration: usize,
}

impl Directions {
    fn new(directions: Vec<Direction>) -> Directions {
        Directions {
            directions,
            direction_next: 0,
            iteration: 0,
        }
    }

    fn next(&mut self) -> Direction {
        if self.direction_next == self.directions.len() {
            self.direction_next = 0;
            self.iteration += 1;
        }

        let direction = self.directions[self.direction_next];
        self.direction_next += 1;
        direction
    }

    fn reset(&mut self) {
        self.direction_next = 0;
        self.iteration = 0;
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "left"),
            Direction::Right => write!(f, "right"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Eq)]
pub(crate) enum Direction {
    Right,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = PuzzleError;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'R' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            _ => Err(PuzzleError::invalid_input(&format!(
                "invalid direction '{value}'"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    pub const INPUT_TWO_STEPS: &str = r"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    pub const INPUT_SIX_STEPS: &str = r"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    pub const GHOST_STEPS: &str = r"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
}

mod part_one {
    use super::{parser::parse, Result};

    pub fn solve(input: &str) -> Result<usize> {
        let mut map = parse(input)?;
        Ok(map.steps_count())
    }

    #[cfg(test)]
    pub mod tests {

        use super::{
            super::tests::{INPUT_SIX_STEPS, INPUT_TWO_STEPS},
            parse,
        };

        #[test]
        fn part_one_2_steps() {
            let mut map = parse(INPUT_TWO_STEPS).expect("valid parse");
            assert_eq!(2, map.steps_count())
        }

        #[test]
        fn part_one_6_steps() {
            let mut map = parse(INPUT_SIX_STEPS).expect("valid parse");
            assert_eq!(6, map.steps_count())
        }
    }
}

mod part_two {
    use super::{parser::parse, Result};

    pub fn solve(input: &str) -> Result<usize> {
        let mut map = parse(input)?;
        Ok(map.ghost_steps_count())
    }

    #[cfg(test)]
    pub mod tests {
        use super::{super::tests::GHOST_STEPS, parse};

        #[test]
        fn part_two() {
            let mut map = parse(GHOST_STEPS).expect("valid parse");
            assert_eq!(6, map.ghost_steps_count())
        }
    }
}

mod parser {
    use std::collections::HashMap;

    use super::{Direction, GuidedMap, PuzzleError, Result};
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref RE_CARD_TOKEN: Regex =
            Regex::new(r"(?<node>[A-Z0-9]{3}) = \((?<left>[A-Z0-9]{3}), (?<right>[A-Z0-9]{3})\)")
                .expect("failed node steps regex");
    }

    pub(super) fn parse(input: &str) -> Result<GuidedMap> {
        let (directions, network) = input
            .lines()
            .enumerate()
            .filter_map(filter_empty_line_input)
            .try_fold(
                (None, HashMap::new()),
                |(directions, mut network), (line, input)| {
                    if let Some(directions) = directions {
                        let captures = RE_CARD_TOKEN.captures(input).ok_or_else(|| {
                            PuzzleError::invalid_line_input(line, "invalid node steps format")
                        })?;
                        let node = captures.name("node").expect("captures succeeded").as_str();
                        let left = captures.name("left").expect("captures succeeded").as_str();
                        let right = captures.name("right").expect("captures succeeded").as_str();
                        network.insert(node.to_owned(), (left.to_owned(), right.to_owned()));
                        Ok((Some(directions), network))
                    } else {
                        match input
                            .chars()
                            .map(Direction::try_from)
                            .collect::<Result<Vec<_>>>()
                        {
                            Ok(directions) => Ok((Some(directions), network)),
                            Err(err) => Err(err),
                        }
                    }
                },
            )
            .and_then(|(directions, network)| {
                if let Some(directions) = directions {
                    Ok((directions, network))
                } else {
                    Err(PuzzleError::invalid_input("no directions specified"))
                }
            })?;
        Ok(GuidedMap::new(directions, network))
    }

    fn filter_empty_line_input((line, input): (usize, &str)) -> Option<(usize, &str)> {
        let input = input.trim();
        if input.is_empty() {
            None
        } else {
            Some((line, input))
        }
    }

    #[cfg(test)]
    mod tests {

        use super::{super::tests::INPUT_TWO_STEPS, parse, Direction::*, GuidedMap};

        #[test]
        fn parse_test() {
            let expected_map = GuidedMap::new(
                vec![Right, Left],
                [
                    node_paths("AAA", "BBB", "CCC"),
                    node_paths("BBB", "DDD", "EEE"),
                    node_paths("CCC", "ZZZ", "GGG"),
                    node_paths("DDD", "DDD", "DDD"),
                    node_paths("EEE", "EEE", "EEE"),
                    node_paths("GGG", "GGG", "GGG"),
                    node_paths("ZZZ", "ZZZ", "ZZZ"),
                ]
                .into_iter()
                .collect(),
            );
            assert_eq!(Ok(expected_map), parse(INPUT_TWO_STEPS));
        }

        fn node_paths(node: &str, left: &str, right: &str) -> (String, (String, String)) {
            (node.to_owned(), (left.to_owned(), right.to_owned()))
        }
    }
}
