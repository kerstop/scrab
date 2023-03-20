use serde::{Serialize, Deserialize};

use crate::hex_grid::{HexGrid, Cordinate};

#[derive(Serialize, Deserialize)]
pub struct World {
    rooms: HexGrid<Room>,
}

impl World {
    pub fn new(size: i32) -> Self {
        World { rooms: HexGrid::new(size) }
    }

    pub fn get(&self, cord: &Cordinate) -> Option<&Room> {
        self.rooms.get(cord)
    }

    pub fn get_mut(&mut self, cord: &Cordinate) -> Option<&mut Room> {
        self.rooms.get_mut(cord)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Room {
    tiles: crate::hex_grid::HexGrid<Tile>,
}

impl Room {
    pub fn get(&self, cord: &Cordinate) -> Option<&Tile> {
        self.tiles.get(cord)
    }

    pub fn get_mut(&mut self, cord: &Cordinate) -> Option<&mut Tile> {
        self.tiles.get_mut(cord)
    }
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
