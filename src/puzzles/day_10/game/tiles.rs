use crate::{PuzzleError, Result};

use std::{collections::HashSet, fmt::Debug, fmt::Display};

mod tiles_iter;
use tiles_iter::{EnclosedTileIter, PathIter};

const MIN_TILES_HEIGHT_WIDTH: usize = 3;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct TilePosition {
    tile: Tile,
    row: usize,
    column: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub(crate) enum Tile {
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
}

impl Tile {
    pub(crate) fn is_start(&self) -> bool {
        matches!(self, &Tile::Start)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Tile::*;

        let display = match self {
            Start => "S",
            Vertical => "|",
            Horizontal => "-",
            NorthEast => "L",
            NorthWest => "J",
            SouthWest => "7",
            SouthEast => "F",
            Ground => ".",
        };
        write!(f, "{display}")
    }
}

impl TryFrom<char> for Tile {
    type Error = PuzzleError;

    fn try_from(value: char) -> Result<Tile> {
        use Tile::*;

        match value {
            'S' => Ok(Start),
            '|' => Ok(Vertical),
            '-' => Ok(Horizontal),
            'L' => Ok(NorthEast),
            'J' => Ok(NorthWest),
            '7' => Ok(SouthWest),
            'F' => Ok(SouthEast),
            '.' => Ok(Ground),
            _ => Err(PuzzleError::invalid_input(&format!(
                "invalid tile char '{value}'"
            ))),
        }
    }
}

impl TryFrom<&char> for Tile {
    type Error = PuzzleError;

    fn try_from(value: &char) -> Result<Tile> {
        Tile::try_from(*value)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PuzzleError, Result,
        Tile::{self, *},
    };

    #[test]
    fn tile_try_from() {
        assert_eq!(
            vec![
                Ok(Start),
                Ok(Vertical),
                Ok(Horizontal),
                Ok(NorthEast),
                Ok(NorthWest),
                Ok(SouthWest),
                Ok(SouthEast),
                Ok(Ground),
                Err(PuzzleError::invalid_input("invalid tile char 'X'")),
            ],
            ['S', '|', '-', 'L', 'J', '7', 'F', '.', 'X']
                .iter()
                .map(Tile::try_from)
                .collect::<Vec<Result<Tile>>>()
        );
    }
}

#[derive(PartialEq)]
pub(crate) struct Tiles {
    tiles: Vec<Vec<Tile>>,
    last_row: usize,
    last_column: usize,
    pipe_start_row: usize,
    pipe_start_column: usize,
}

impl Tiles {
    pub fn path_iter(&self) -> PathIter {
        PathIter::new(self)
    }

    pub fn enclosed_tile_iter(&self) -> EnclosedTileIter {
        EnclosedTileIter::new(self)
    }

    pub fn write(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pipe_path = self.path_iter().collect::<HashSet<_>>();
        let enclosed_tiles = self.enclosed_tile_iter().collect::<HashSet<_>>();
        for row in 0..=self.last_row {
            for column in 0..=self.last_column {
                let tile_pos = self.get_tile_pos(row, column);
                if enclosed_tiles.contains(&tile_pos) {
                    write!(f, "I")?;
                } else if pipe_path.contains(&tile_pos) {
                    write!(f, "{}", tile_pos.tile)?;
                } else {
                    write!(f, "O")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }

    fn get_tile_pos_movable_positions(&self, tile_pos: &TilePosition) -> Vec<TilePosition> {
        self.get_position_movable_positions(tile_pos.row, tile_pos.column)
            .into_iter()
            .map(|(row, column)| self.get_tile_pos(row, column))
            .collect()
    }

    fn get_tile(&self, row: usize, column: usize) -> Tile {
        self.tiles[row][column]
    }

    fn get_tile_pos(&self, row: usize, column: usize) -> TilePosition {
        let tile = self.get_tile(row, column);
        TilePosition { tile, row, column }
    }

    fn get_position_movable_positions(&self, row: usize, column: usize) -> Vec<(usize, usize)> {
        use Tile::*;

        let tile = self.get_tile(row, column);
        let mut movable_positions = Vec::with_capacity(2);
        if row > 0
            && matches!(tile, Vertical | NorthEast | NorthWest)
            && matches!(
                self.tiles[row - 1][column],
                Vertical | SouthEast | SouthWest
            )
        {
            movable_positions.push((row - 1, column));
        }
        if column < self.last_column
            && matches!(tile, Horizontal | NorthEast | SouthEast)
            && matches!(
                self.tiles[row][column + 1],
                Horizontal | NorthWest | SouthWest
            )
        {
            movable_positions.push((row, column + 1));
        }
        if row < self.last_row
            && matches!(tile, Vertical | SouthEast | SouthWest)
            && matches!(
                self.tiles[row + 1][column],
                Vertical | NorthEast | NorthWest
            )
        {
            movable_positions.push((row + 1, column));
        }
        if column > 0
            && matches!(tile, Horizontal | NorthWest | SouthWest)
            && matches!(
                self.tiles[row][column - 1],
                Horizontal | NorthEast | SouthEast
            )
        {
            movable_positions.push((row, column - 1));
        }
        assert!(movable_positions.len() == 2);
        movable_positions
    }

    fn validate_tiles(tiles: &[Vec<Tile>]) -> Result<TilePosition> {
        if tiles.len() < MIN_TILES_HEIGHT_WIDTH {
            return Err(PuzzleError::invalid_input("the tiles is too short"));
        }

        let (start_tile_pos, width) = tiles.iter().enumerate().try_fold(
            (None::<TilePosition>, None::<usize>),
            |(start_tile_pos, width), (row, row_tiles)| {
                if let Some(width) = width {
                    if row_tiles.len() != width {
                        return Err(PuzzleError::invalid_input(
                            "all tile vectors must have the same length",
                        ));
                    }
                }

                let start_tile_poses = row_tiles
                    .iter()
                    .enumerate()
                    .filter_map(|(column, tile)| {
                        if tile.is_start() {
                            Some(TilePosition {
                                tile: *tile,
                                row,
                                column,
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<TilePosition>>();
                let width = row_tiles.len();
                if start_tile_poses.len() == 1 {
                    if start_tile_pos.is_some() {
                        return Err(PuzzleError::invalid_input("too many start tiles"));
                    }
                    return Ok((Some(start_tile_poses[0]), Some(width)));
                }
                Ok((start_tile_pos, Some(width)))
            },
        )?;

        if width.expect("there's at least min rows") < MIN_TILES_HEIGHT_WIDTH {
            return Err(PuzzleError::invalid_input("the tiles is too narrow"));
        }

        if let Some(start_tile_pos) = start_tile_pos {
            Ok(start_tile_pos)
        } else {
            Err(PuzzleError::invalid_input(
                "the tile vectors must have a single start",
            ))
        }
    }

    fn resolve_pipe_start_tile(&mut self) {
        use Tile::*;

        let row = self.pipe_start_row;
        let column = self.pipe_start_column;
        let mut connect_north = false;
        let mut connect_east = false;
        let mut connect_south = false;
        let mut connect_west = false;

        if row > 0 {
            let tile = self.tiles[row - 1][column];
            connect_north = matches!(tile, Vertical | SouthEast | SouthWest);
        }
        if column < self.last_column {
            let tile = self.tiles[row][column + 1];
            connect_east = matches!(tile, Horizontal | NorthWest | SouthWest);
        }
        if row < self.last_row {
            let tile = self.tiles[row + 1][column];
            connect_south = matches!(tile, Vertical | NorthEast | NorthWest);
        }
        if column > 0 {
            let tile = self.tiles[row][column - 1];
            connect_west = matches!(tile, Horizontal | NorthEast | SouthEast);
        }
        self.tiles[row][column] = match (connect_north, connect_east, connect_south, connect_west) {
            (true, true, false, false) => NorthEast,
            (true, false, true, false) => Vertical,
            (true, false, false, true) => NorthWest,
            (false, true, true, false) => SouthEast,
            (false, true, false, true) => Horizontal,
            (false, false, true, true) => SouthWest,
            _ => panic!("unsupported pipe connection pattern"),
        };
    }
}

impl Display for Tiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write(f)
    }
}

impl Debug for Tiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write(f)
    }
}

impl TryFrom<Vec<Vec<Tile>>> for Tiles {
    type Error = PuzzleError;

    fn try_from(tiles: Vec<Vec<Tile>>) -> Result<Self> {
        let start_tile_pos = Tiles::validate_tiles(&tiles)?;
        let last_row = tiles.len() - 1;
        let last_column = tiles[0].len() - 1;
        let mut tiles = Tiles {
            tiles,
            last_row,
            last_column,
            pipe_start_row: start_tile_pos.row,
            pipe_start_column: start_tile_pos.column,
        };
        tiles.resolve_pipe_start_tile();
        Ok(tiles)
    }
}
