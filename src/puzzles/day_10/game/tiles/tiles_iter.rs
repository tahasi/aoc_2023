use std::collections::HashSet;

use super::{Tile, TilePosition, Tiles};

pub(crate) struct PathIter<'a> {
    source: &'a Tiles,
    pipe_start_row: usize,
    pipe_start_column: usize,
    prior_tile_pos: Option<TilePosition>,
    tile_pos: Option<TilePosition>,
}

impl<'a> PathIter<'a> {
    pub(crate) fn new(source: &'a Tiles) -> Self {
        PathIter {
            source,
            pipe_start_row: source.pipe_start_row,
            pipe_start_column: source.pipe_start_column,
            prior_tile_pos: None,
            tile_pos: None,
        }
    }
}

impl<'a> Iterator for PathIter<'a> {
    type Item = TilePosition;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tile_pos) = self.tile_pos {
            // return None if the loop has already been fully iterated
            if tile_pos.row == self.pipe_start_row
                && tile_pos.column == self.pipe_start_column
                && self.prior_tile_pos.is_some()
            {
                return None;
            }

            let next_tile_pos = self
                .source
                .get_tile_pos_movable_positions(&tile_pos)
                .into_iter()
                .find(|next_tile_pos| {
                    if !(next_tile_pos.row == self.pipe_start_row
                        && next_tile_pos.column == self.pipe_start_column)
                    {
                        if let Some(prior_tile_pos) = self.prior_tile_pos {
                            *next_tile_pos != prior_tile_pos
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                });

            self.prior_tile_pos = self.tile_pos;
            if let Some(next_tile_pos) = next_tile_pos {
                self.tile_pos = Some(next_tile_pos);
                self.tile_pos
            } else {
                self.tile_pos = Some(
                    self.source
                        .get_tile_pos(self.source.pipe_start_row, self.source.pipe_start_column),
                );
                None
            }
        } else {
            self.tile_pos = Some(
                self.source
                    .get_tile_pos(self.source.pipe_start_row, self.source.pipe_start_column),
            );
            self.tile_pos
        }
    }
}

pub(crate) struct EnclosedTileIter<'a> {
    source: &'a Tiles,
    pipe_path_tiles: HashSet<TilePosition>,
    exterior: bool,
    pipe_run_start_tile: Option<Tile>,
    position: Option<(usize, usize)>,
}

impl<'a> EnclosedTileIter<'a> {
    pub(crate) fn new(source: &'a Tiles) -> Self {
        let pipe_path_tiles: HashSet<_> = source.path_iter().collect();
        EnclosedTileIter {
            source,
            pipe_path_tiles,
            exterior: true,
            pipe_run_start_tile: None,
            position: None,
        }
    }
}

impl<'a> Iterator for EnclosedTileIter<'a> {
    type Item = TilePosition;

    fn next(&mut self) -> Option<Self::Item> {
        use super::Tile::*;
        let mut position = match self.position {
            Some((mut row, mut column)) => {
                column += 1;
                if column > self.source.last_column {
                    column = 0;
                    row += 1;
                    self.exterior = true;
                }
                (row, column)
            }
            None => (0, 0),
        };

        for row in position.0..=self.source.last_row {
            for column in position.1..=self.source.last_column {
                let tile_pos = self.source.get_tile_pos(row, column);
                let is_pipe_tile = self.pipe_path_tiles.contains(&tile_pos);
                if !is_pipe_tile {
                    if !self.exterior {
                        self.position = Some((row, column));
                        return Some(tile_pos);
                    }
                    continue;
                }
                if let Vertical = tile_pos.tile {
                    self.exterior = !self.exterior;
                    continue;
                }
                if let Horizontal = tile_pos.tile {
                    continue;
                }
                if let Some(pipe_run_start_tile) = self.pipe_run_start_tile {
                    match (pipe_run_start_tile, tile_pos.tile) {
                        (NorthEast, SouthWest) => self.exterior = !self.exterior,
                        (SouthEast, NorthWest) => self.exterior = !self.exterior,
                        (NorthEast, NorthWest) => {}
                        (SouthEast, SouthWest) => {}
                        _ => panic!("shouldn't happen"),
                    }
                    self.pipe_run_start_tile = None;
                    continue;
                }
                match tile_pos.tile {
                    NorthEast | SouthEast => self.pipe_run_start_tile = Some(tile_pos.tile),
                    _ => panic!("shouldn't happen either"),
                }
            }
            position.1 = 0;
            self.exterior = true;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{TilePosition, Tiles};

    use super::super::Tile::{self, *};

    fn tile_pos(tile: Tile, row: usize, column: usize) -> TilePosition {
        TilePosition { tile, row, column }
    }

    #[test]
    fn path_iter() {
        let tiles = Tiles::try_from(vec![
            vec![SouthEast, Horizontal, SouthWest],
            vec![Start, SouthEast, NorthWest],
            vec![NorthEast, NorthWest, NorthWest],
        ])
        .expect("valid vec");

        let mut iter = tiles.path_iter();

        assert_eq!(Some(tile_pos(Vertical, 1, 0)), iter.next());
        assert_eq!(Some(tile_pos(SouthEast, 0, 0)), iter.next());
        assert_eq!(Some(tile_pos(Horizontal, 0, 1)), iter.next());
        assert_eq!(Some(tile_pos(SouthWest, 0, 2)), iter.next());
        assert_eq!(Some(tile_pos(NorthWest, 1, 2)), iter.next());
        assert_eq!(Some(tile_pos(SouthEast, 1, 1)), iter.next());
        assert_eq!(Some(tile_pos(NorthWest, 2, 1)), iter.next());
        assert_eq!(Some(tile_pos(NorthEast, 2, 0)), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }
}
