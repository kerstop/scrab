use scrab_public_types::*;
use serde::{Deserialize, Serialize};

use hex_grid::{Cordinate, HexGrid};

#[derive(Serialize, Deserialize)]
pub struct World {
    pub(crate) rooms: HexGrid<Room>,
}

impl From<&World> for PublicWorld {
    fn from(world: &World) -> Self {
        let mut rooms = Vec::new();

        for room in world.rooms.cordinates() {
            rooms.push(room.to_string())
        }

        PublicWorld { rooms }
    }
}

impl World {
    pub fn new(size: i32) -> Self {
        World {
            rooms: HexGrid::new(size),
        }
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
    pub(crate) tiles: hex_grid::HexGrid<Tile>,
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
        Self {
            tiles: HexGrid::new(21),
        }
    }
}

impl From<&Room> for PublicRoom {
    fn from(room: &Room) -> Self {
        let mut tiles = Vec::new();

        for cord in room.tiles.cordinates() {
            if let Some(tile) = room.tiles.get(&cord) {
                let (x, y) = cord.to_pixel(100.0);
                tiles.push(PublicTile {
                    wall: tile.wall,
                    name: cord.to_string(),
                    x,
                    y,
                })
            }
        }

        PublicRoom { tiles }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Tile {
    pub(crate) wall: bool,
}
