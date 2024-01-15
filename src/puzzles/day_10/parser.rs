use super::game::{
    tiles::{Tile, Tiles},
    Map,
};
use crate::result::{PuzzleError, Result};

pub fn parse(input: &str) -> Result<Map> {
    Ok(Map::new(Tiles::try_from(
        input
            .trim()
            .lines()
            .map(|input| {
                input
                    .trim()
                    .chars()
                    .map(Tile::try_from)
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<Vec<_>>>>()
            .map_err(|_| PuzzleError::invalid_input("invalid map tiles"))?,
    )?))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::{Map, Tile::*, Tiles};

        const INPUT: &str = "
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let expected_map = Map::new(
            Tiles::try_from(vec![
                vec![SouthWest, Horizontal, SouthEast, SouthWest, Horizontal],
                vec![Ground, SouthEast, NorthWest, Vertical, SouthWest],
                vec![Start, NorthWest, NorthEast, NorthEast, SouthWest],
                vec![Vertical, SouthEast, Horizontal, Horizontal, NorthWest],
                vec![NorthEast, NorthWest, Ground, NorthEast, NorthWest],
            ])
            .expect("valid map"),
        );

        assert_eq!(Ok(expected_map), super::parse(INPUT));
    }
}
