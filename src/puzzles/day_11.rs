use std::str::FromStr;

use super::{read_input_file, PuzzleError, Result};

pub fn run_part_one() -> Result<()> {
    let input = read_input_file(11, None)?;
    let sum = part_one::solve(input.trim())?;
    println!("Day eleven part one: sum of galaxy path lengths: {sum}");
    Ok(())
}

pub fn run_part_two() -> Result<()> {
    let input = read_input_file(11, None)?;
    let sum = part_two::solve(input.trim())?;
    println!("Day eleven part one: sum of older galaxy path lengths: {sum}");
    Ok(())
}

struct Universe {
    galaxies: Vec<(usize, usize)>,
    last_row: usize,
    last_column: usize,
}

fn galaxy_path_length(from: &(usize, usize), to: &(usize, usize)) -> usize {
    let x_distance = from.0.abs_diff(to.0);
    let y_distance = from.1.abs_diff(to.1);
    x_distance + y_distance
}

fn expand_indices(has_galaxies: &[bool], rate: usize) -> Result<Vec<usize>> {
    if rate == 0 {
        return Err(PuzzleError::unexpected(
            "expansion rate must be 1 or greater",
        ));
    }

    let expansion_increment = rate - 1;
    let mut expansion = 0;
    Ok(has_galaxies
        .iter()
        .enumerate()
        .map(|(index, &has_galaxies)| {
            if has_galaxies {
                index + expansion
            } else {
                expansion += expansion_increment;
                0
            }
        })
        .collect::<Vec<_>>())
}

impl Universe {
    pub fn galaxies_paths_iter(
        &self,
    ) -> impl Iterator<Item = ((usize, usize), (usize, usize), usize)> + '_ {
        self.galaxies[0..self.galaxies.len() - 1]
            .iter()
            .copied()
            .enumerate()
            .flat_map(|(index, from)| {
                self.galaxies[(index + 1)..self.galaxies.len()]
                    .iter()
                    .copied()
                    .map(move |to| (from, to, galaxy_path_length(&from, &to)))
            })
    }

    pub fn expand(&mut self, rate: usize) -> Result<()> {
        let mut rows_has_galaxies = vec![false; self.last_row + 1];
        let mut columns_has_galaxies = vec![false; self.last_column + 1];
        self.galaxies.iter().for_each(|&(row, column)| {
            rows_has_galaxies[row] = true;
            columns_has_galaxies[column] = true;
        });
        let expanded_row_indices = expand_indices(&rows_has_galaxies, rate)?;
        let expanded_column_indices = expand_indices(&columns_has_galaxies, rate)?;
        self.galaxies = self
            .galaxies
            .iter()
            .map(|&(row, column)| (expanded_row_indices[row], expanded_column_indices[column]))
            .collect();
        Ok(())
    }
}

impl FromStr for Universe {
    type Err = PuzzleError;

    fn from_str(input: &str) -> Result<Self> {
        let galaxies = input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(row, input)| {
                input
                    .trim()
                    .chars()
                    .enumerate()
                    .map(move |(column, input)| match input {
                        '#' => Ok(Some((row, column))),
                        '.' => Ok(None),
                        other => Err(PuzzleError::invalid_line_input(
                            row,
                            &format!("invalid character '{}'", other),
                        )),
                    })
            })
            .collect::<Result<Vec<Option<(usize, usize)>>>>()?;
        let galaxies = galaxies
            .into_iter()
            .flatten()
            .collect::<Vec<(usize, usize)>>();

        let (last_row, last_column) = galaxies.iter().fold(
            (0usize, 0usize),
            |(last_row, last_column), (row, column)| (last_row.max(*row), last_column.max(*column)),
        );
        Ok(Universe {
            galaxies,
            last_row,
            last_column,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Universe;
    use std::str::FromStr;

    const INPUT: &str = r"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part_two_solve() {
        let sum = super::part_two::solve(INPUT);
        assert_eq!(sum, Ok(374));
    }

    #[test]
    fn part_one_solve() {
        let sum = super::part_one::solve(INPUT);
        assert_eq!(sum, Ok(374));
    }

    #[test]
    fn universe_expand_10_sum() {
        let mut universe = Universe::from_str(INPUT).expect("valid input");
        universe.expand(10).expect("valid expansion");
        assert_eq!(
            universe
                .galaxies_paths_iter()
                .map(|(_, _, distance)| distance as u64)
                .sum::<u64>(),
            1030u64
        );
    }

    #[test]
    fn universe_galaxies_iter() {
        let universe = Universe::from_str(INPUT).expect("valid input");
        assert_eq!(
            universe.galaxies,
            vec![
                (0, 3),
                (1, 7),
                (2, 0),
                (4, 6),
                (5, 1),
                (6, 9),
                (8, 7),
                (9, 0),
                (9, 4),
            ]
        );
    }

    #[test]
    fn universe_expand() {
        let mut universe = Universe::from_str(INPUT).expect("valid input");
        universe.expand(2).expect("valid expansion");
        assert_eq!(
            universe.galaxies,
            vec![
                (0, 4),
                (1, 9),
                (2, 0),
                (5, 8),
                (6, 1),
                (7, 12),
                (10, 9),
                (11, 0),
                (11, 5),
            ]
        );
    }

    #[test]
    fn universe_galaxies_paths_iter() {
        let universe = Universe::from_str(INPUT).expect("valid input");
        let mut galaxy_distances = universe.galaxies_paths_iter();
        assert_eq!(galaxy_distances.next(), Some(((0, 3), (1, 7), 5)));
        assert_eq!(galaxy_distances.next(), Some(((0, 3), (2, 0), 5)));
        assert_eq!(galaxy_distances.next(), Some(((0, 3), (4, 6), 7)));
        assert_eq!(galaxy_distances.next(), Some(((0, 3), (5, 1), 7)));
        assert_eq!(galaxy_distances.next(), Some(((0, 3), (6, 9), 12)));
        assert_eq!(galaxy_distances.next(), Some(((0, 3), (8, 7), 12)));
        assert_eq!(galaxy_distances.next(), Some(((0, 3), (9, 0), 12)));
        assert_eq!(galaxy_distances.next(), Some(((0, 3), (9, 4), 10)));
        assert_eq!(galaxy_distances.next(), Some(((1, 7), (2, 0), 8)));
        let mut galaxy_distances = galaxy_distances.skip(6);
        assert_eq!(galaxy_distances.next(), Some(((2, 0), (4, 6), 8)));
        let mut galaxy_distances = galaxy_distances.skip(5);
        assert_eq!(galaxy_distances.next(), Some(((4, 6), (5, 1), 6)));
        let mut galaxy_distances = galaxy_distances.skip(4);
        assert_eq!(galaxy_distances.next(), Some(((5, 1), (6, 9), 9)));
        let mut galaxy_distances = galaxy_distances.skip(3);
        assert_eq!(galaxy_distances.next(), Some(((6, 9), (8, 7), 4)));
        let mut galaxy_distances = galaxy_distances.skip(2);
        assert_eq!(galaxy_distances.next(), Some(((8, 7), (9, 0), 8)));
        let mut galaxy_distances = galaxy_distances.skip(1);
        assert_eq!(galaxy_distances.next(), Some(((9, 0), (9, 4), 4)));
        assert_eq!(galaxy_distances.next(), None);
    }
}

mod part_one {
    use super::{Result, Universe};

    pub fn solve(input: &str) -> Result<u64> {
        let mut universe: Universe = input.parse()?;
        universe.expand(2)?;
        let sum = universe
            .galaxies_paths_iter()
            .map(|(_, _, distance)| distance as u64)
            .sum();
        Ok(sum)
    }
}

mod part_two {
    use super::{Result, Universe};

    pub fn solve(input: &str) -> Result<u64> {
        let mut universe: Universe = input.parse()?;
        universe.expand(1_000_000)?;
        let sum = universe
            .galaxies_paths_iter()
            .map(|(_, _, distance)| distance as u64)
            .sum();
        Ok(sum)
    }
}
