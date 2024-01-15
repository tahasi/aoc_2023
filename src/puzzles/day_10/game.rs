use std::fmt::{Debug, Display};

pub(crate) mod tiles;
use tiles::Tiles;

#[derive(PartialEq)]
pub(crate) struct Map {
    tiles: Tiles,
}

impl Map {
    pub(crate) fn new(tiles: Tiles) -> Map {
        Map { tiles }
    }

    pub(crate) fn steps_to_furthest_point(&self) -> u64 {
        (self.tiles.path_iter().count() as u64 + 1) / 2
    }

    pub(crate) fn enclosed_tile_count(&self) -> u64 {
        println!("{self}");
        self.tiles.enclosed_tile_iter().count() as u64
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tiles.write(f)
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tiles.write(f)
    }
}
