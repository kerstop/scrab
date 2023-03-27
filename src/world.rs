use scrab_public_types::*;
use serde::{Deserialize, Serialize};

use hex_grid::{Cordinate, HexGrid};

const ROOM_SIZE: i32 = 20;
const WORLD_SIZE: i32 = 20;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub rooms: HexGrid<Room>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_pub(&self) -> PublicWorld {
        let mut rooms: Vec<PublicRoomManifest> = Vec::new();

        for room_cord in self.rooms.cordinates() {
            let (x, y) = room_cord
                .to_pixel_point(100.0 * 3.0f64.sqrt() * (ROOM_SIZE + 1) as f64);
            rooms.push(PublicRoomManifest {
                name: room_cord.to_string(),
                screen_space_x: x,
                screen_space_y: y,
            })
        }

        PublicWorld { rooms }
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            rooms: HexGrid::new(WORLD_SIZE),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Room {
    pub tiles: hex_grid::HexGrid<Tile>,
}

impl Default for Room {
    fn default() -> Self {
        Self {
            tiles: HexGrid::new(ROOM_SIZE),
        }
    }
}

impl From<&Room> for PublicRoom {
    fn from(room: &Room) -> Self {
        let mut tiles = Vec::new();

        for cord in room.tiles.cordinates() {
            if let Some(tile) = room.tiles.get(cord) {
                let (x, y) = cord.to_pixel_flat(100.0);
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
    pub wall: bool,
}
