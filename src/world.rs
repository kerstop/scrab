use codegen::cord;
use noise::*;
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
        let mut world = Self::default();
        world.setup();
        world
    }

    pub fn to_pub(&self) -> PublicWorld {
        let mut rooms: Vec<PublicRoomManifest> = Vec::new();

        for room_cord in self.rooms.cordinates() {
            let (x, y) = room_cord.to_pixel_point(100.0 * 3.0f64.sqrt() * (ROOM_SIZE + 1) as f64);
            rooms.push(PublicRoomManifest {
                name: room_cord.to_string(),
                screen_space_x: x,
                screen_space_y: y,
            })
        }

        PublicWorld { rooms }
    }

    pub fn setup(&mut self) {
        let wall_threshhold = 0.1;
        let noise = Perlin::new(7);

        for (room, room_cord) in self.rooms.iter_mut() {
            for (tile, tile_cord) in room.tiles.iter_mut() {
                let (x, y) = tile_to_worldspace(tile_cord, room_cord, 0.08);
                let value = noise.get([x, y]);
                tile.wall = value > wall_threshhold;
            }
        }
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

pub fn tile_to_worldspace(tile: Cordinate, room: Cordinate, scale: f64) -> (f64, f64) {
    let (tile_x, tile_y) = tile.to_pixel_flat(scale);
    let (room_x, room_y) = room.to_pixel_point(scale * 3.0f64.sqrt() * ROOM_SIZE as f64);
    (tile_x + room_x, tile_y + room_y)
}
