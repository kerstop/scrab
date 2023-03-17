use serde::{Serialize, Deserialize};

use crate::hex_grid::HexGrid;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub(crate) rooms: HexGrid<Room>,
}

impl World {
    pub fn new(size: i32) -> Self {
        World { rooms: HexGrid::new(size) }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Room {
    pub(crate) tiles: crate::hex_grid::HexGrid<Tile>,
}

impl Default for Room {
    fn default() -> Self {
        Self { tiles: HexGrid::new(31) }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub(crate) wall: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Self { wall: false }
    }
}
