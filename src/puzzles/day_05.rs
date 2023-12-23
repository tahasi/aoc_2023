use std::ops::Range;

use log::debug;

use crate::{read_input_file, PuzzleError, Result};

pub fn run_part_one() -> Result<()> {
    let input = read_input_file(5, None)?;
    let location = part_one::solve(input.trim())?;
    println!("Day five part one: lowest location number: {location}");
    Ok(())
}

pub fn run_part_two() -> Result<()> {
    let input = read_input_file(5, None)?;
    let location = part_two::solve(input.trim())?;
    println!("Day five part two: lowest location number: {location}");
    Ok(())
}

enum SeedMode {
    Independent,
    Range,
}

struct Almanac {
    seed_ranges: Vec<Range<u64>>,
    component_maps: Vec<ComponentMap>,
}

impl Almanac {
    fn seeds(&self) -> impl Iterator<Item = u64> + '_ {
        self.seed_ranges.iter().flat_map(|range| range.to_owned())
    }

    fn seeds_count(&self) -> u64 {
        self.seed_ranges
            .iter()
            .map(|range| range.end - range.start)
            .sum()
    }

    fn lowest_location(&self) -> u64 {
        let count = self.seeds_count();
        let report_progress_interval = count / 100 + 1;
        debug!("finding lowest location of {} seeds", count);
        self.seeds()
            .enumerate()
            .map(|(progress, id)| {
                if progress as u64 % report_progress_interval == 0 {
                    debug!(
                        "{}/{} {}%",
                        progress,
                        count,
                        (progress as f64 / count as f64 * 100f64) as u32
                    );
                }
                return self.component_maps.iter().fold(id, |id, map| map.map(id));
            })
            .min()
            .expect("must have mapping to end of component maps")
    }
}

#[derive(Debug)]
struct ComponentMap {
    #[allow(dead_code)]
    name: String,
    maps: Vec<RangeMap>,
}

impl ComponentMap {
    fn map(&self, id: u64) -> u64 {
        if let Some(map) = self
            .maps
            .iter()
            .find(|map| map.source <= id && id - map.source <= map.length)
        {
            if let Some(destination) = map.destination.checked_add(id - map.source) {
                destination
            } else {
                debug!(
                    "usize overflow adding {} to {}",
                    id - map.source,
                    map.destination
                );
                id
            }
        } else {
            id
        }
    }
}

#[derive(Debug, PartialEq)]
struct RangeMap {
    destination: u64,
    source: u64,
    length: u64,
}

#[cfg(test)]
pub const INPUT: &str = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

mod part_one {
    use super::{parser, Result, SeedMode};

    pub fn solve(input: &str) -> Result<u64> {
        let almanac = parser::parse(input, SeedMode::Independent)?;
        Ok(almanac.lowest_location())
    }

    #[cfg(test)]
    mod tests {
        use super::{super::INPUT, solve};

        #[test]
        fn part_one() {
            assert_eq!(Ok(35), solve(INPUT));
        }
    }
}

mod part_two {
    use super::{parser, Result, SeedMode};

    pub fn solve(input: &str) -> Result<u64> {
        let almanac = parser::parse(input, SeedMode::Range)?;
        Ok(almanac.lowest_location())
    }

    #[cfg(test)]
    mod tests {
        use super::{super::INPUT, solve};

        #[test]
        fn part_two() {
            assert_eq!(Ok(46), solve(INPUT));
        }
    }
}

mod parser {
    use std::{iter::Iterator, ops::Range};

    use log::debug;

    use crate::parser::HasNumbers;

    use super::{Almanac, ComponentMap, PuzzleError, RangeMap, Result, SeedMode};

    pub fn parse(input: &str, seed_mode: SeedMode) -> Result<Almanac> {
        let (mut almanac_builder, component_builder) = input.lines().enumerate().try_fold(
            (AlmanacBuilder::new(seed_mode), None::<ComponentMapBuilder>),
            |(mut almanac_builder, mut component_map_builder), (line, input)| {
                let input = input.trim();
                if input.is_empty() {
                    if let Some(component_map_builder) = component_map_builder {
                        let component_map = component_map_builder.build()?;
                        almanac_builder.add_component_map(component_map);
                        return Ok((almanac_builder, None));
                    }
                    return Ok((almanac_builder, component_map_builder));
                }
                let numbers = input.unsigned_numbers().collect::<Vec<u64>>();
                if numbers.is_empty() {
                    let input = input.split(' ').collect::<Vec<_>>();
                    if input.len() == 2 {
                        return Ok((almanac_builder, Some(ComponentMapBuilder::new(input[0]))));
                    }
                    return Err(PuzzleError::invalid_line_input(line, "invalid map header"));
                }
                if !almanac_builder.has_seeds() {
                    almanac_builder.add_seeds(numbers)?;
                    return Ok((almanac_builder, component_map_builder));
                }
                if let Some(mut component_builder) = component_map_builder.take() {
                    if numbers.len() != 3 {
                        return Err(PuzzleError::invalid_line_input(
                            line,
                            "incorrect map value count",
                        ));
                    }
                    component_builder.add_range(numbers[0], numbers[1], numbers[2]);
                    return Ok((almanac_builder, Some(component_builder)));
                }
                Ok((almanac_builder, component_map_builder))
            },
        )?;
        if let Some(component_builder) = component_builder {
            almanac_builder.add_component_map(component_builder.build()?);
        }
        almanac_builder.build()
    }

    struct AlmanacBuilder {
        seed_mode: SeedMode,
        seed_ranges: Option<Vec<Range<u64>>>,
        component_maps: Option<Vec<ComponentMap>>,
    }

    impl AlmanacBuilder {
        fn new(seed_mode: SeedMode) -> AlmanacBuilder {
            AlmanacBuilder {
                seed_mode,
                seed_ranges: None,
                component_maps: None,
            }
        }

        fn add_seeds(&mut self, seeds: Vec<u64>) -> Result<()> {
            match self.seed_mode {
                SeedMode::Independent => {
                    let seed_ranges = seeds.into_iter().map(|seed| (seed..(seed + 1))).collect();
                    debug!("added seed ranges: {:?}", seed_ranges);
                    self.seed_ranges = Some(seed_ranges);
                    Ok(())
                }
                SeedMode::Range => {
                    if seeds.len() % 2 != 0 {
                        return Err(PuzzleError::unexpected(
                            "invalid seed values for seed ranges",
                        ));
                    }
                    let mut seed_ranges = vec![];
                    for index in (0..seeds.len()).step_by(2) {
                        let start = seeds[index];
                        let end = start + seeds[index + 1];
                        seed_ranges.push(start..end);
                    }
                    debug!("added seed ranges: {:?}", seed_ranges);
                    self.seed_ranges = Some(seed_ranges);
                    Ok(())
                }
            }
        }

        fn add_component_map(&mut self, map: ComponentMap) {
            debug!("adding map '{}'", map.name);
            if let Some(mut component_maps) = self.component_maps.take() {
                component_maps.push(map);
                self.component_maps = Some(component_maps);
            } else {
                self.component_maps = Some(vec![map]);
            }
        }

        fn has_seeds(&self) -> bool {
            self.seed_ranges.is_some()
        }

        fn build(self) -> Result<Almanac> {
            match (self.seed_ranges, self.component_maps) {
                (Some(seeds), Some(component_maps)) => {
                    debug!("built almanac");
                    Ok(Almanac {
                        seed_ranges: seeds,
                        component_maps,
                    })
                }
                _ => Err(PuzzleError::unexpected("invalid state to build Almanac")),
            }
        }
    }

    struct ComponentMapBuilder {
        name: String,
        range_maps: Option<Vec<RangeMap>>,
    }

    impl ComponentMapBuilder {
        fn new(name: &str) -> ComponentMapBuilder {
            ComponentMapBuilder {
                name: name.to_owned(),
                range_maps: None,
            }
        }

        fn add_range(&mut self, destination: u64, source: u64, length: u64) {
            let map = RangeMap {
                destination,
                source,
                length,
            };
            if let Some(mut range_maps) = self.range_maps.take() {
                range_maps.push(map);
                self.range_maps = Some(range_maps);
            } else {
                self.range_maps = Some(vec![map]);
            }
        }

        fn build(self) -> Result<ComponentMap> {
            if let Some(range_maps) = self.range_maps {
                Ok(ComponentMap {
                    name: self.name,
                    maps: range_maps,
                })
            } else {
                Err(PuzzleError::unexpected(
                    "invalid state to build ComponentMap",
                ))
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{super::INPUT, parse, Almanac, ComponentMap, RangeMap, SeedMode};

        #[test]
        fn parse_seed_mode_independent() {
            let almanac = parse(INPUT, SeedMode::Independent).expect("failed parse");
            assert_eq!(vec![79, 14, 55, 13], almanac.seeds().collect::<Vec<_>>());
            assert_eq!(
                vec![range(50, 98, 2), range(52, 50, 48)],
                component_map(&almanac, "seed-to-soil").maps
            );
            assert_eq!(
                vec![range(60, 56, 37), range(56, 93, 4)],
                component_map(&almanac, "humidity-to-location").maps
            );
        }

        #[test]
        fn parse_seed_mode_range() {
            let almanac = parse(INPUT, SeedMode::Range).expect("failed parse");
            let mut seeds = seeds_from_range(79, 14);
            seeds.append(&mut seeds_from_range(55, 13));
            assert_eq!(seeds, almanac.seeds().collect::<Vec<_>>());
            assert_eq!(
                vec![range(50, 98, 2), range(52, 50, 48)],
                component_map(&almanac, "seed-to-soil").maps
            );
            assert_eq!(
                vec![range(60, 56, 37), range(56, 93, 4)],
                component_map(&almanac, "humidity-to-location").maps
            );
        }

        fn component_map<'a>(almanac: &'a Almanac, name: &'a str) -> &'a ComponentMap {
            if let Some(map) = almanac.component_maps.iter().find(|map| map.name == name) {
                map
            } else {
                panic!("missing ComponentMap named '{name}'")
            }
        }

        fn range(destination: u64, source: u64, length: u64) -> RangeMap {
            RangeMap {
                destination,
                source,
                length,
            }
        }

        fn seeds_from_range(start: u64, length: u64) -> Vec<u64> {
            (start..(start + length)).into_iter().collect()
        }
    }
}
