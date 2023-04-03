use hex_grid::HexGrid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct World {
    pub rooms: HexGrid<Room>,
    pub room_size: i32,
    pub world_size: i32,
    pub current_tick: u64,
}

impl World {
    pub fn new(world_size: i32, room_size: i32) -> Self {
        let room_template = Room::new(room_size);
        World {
            rooms: HexGrid::from_template(room_template.clone(), world_size),
            room_size,
            world_size,
            current_tick: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Room {
    pub tiles: hex_grid::HexGrid<Tile>,
}

impl Room {
    pub fn new(size: i32) -> Self {
        Room {
            tiles: HexGrid::new(size),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Tile {
    pub wall: bool,
}