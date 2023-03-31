mod world_gen;

use clap::Parser;
use noise::{Perlin, NoiseFn};
use scrab_public_types::*;
use serde::{Deserialize, Serialize};

use hex_grid::{Cordinate, HexGrid};

#[derive(Serialize, Deserialize)]
pub struct World {
    pub rooms: HexGrid<Room>,
    gen_settings: WorldGenerationSettings,
}

impl World {
    pub fn new() -> Self {
        let mut settings = WorldGenerationSettings::default();
        settings.seed = rand::random();
        settings.into()
    }

    pub fn gen_settings(&self) -> &WorldGenerationSettings {
        &self.gen_settings
    }

    pub fn to_pub(&self) -> PublicWorld {
        let mut rooms: Vec<PublicRoomManifest> = Vec::new();

        for room_cord in self.rooms.cordinates() {
            let (x, y) = room_cord
                .to_pixel_point(100.0 * 3.0f64.sqrt() * (self.gen_settings.room_size + 1) as f64);
            rooms.push(PublicRoomManifest {
                name: room_cord.to_string(),
                screen_space_x: x,
                screen_space_y: y,
            })
        }

        PublicWorld { rooms }
    }

    fn generate_walls(&mut self) {
        let wall_threshhold = self.gen_settings.wall_threshold;
        let noise = Perlin::new(self.gen_settings.seed as u32);
        let noise_scale = self.gen_settings.noise_scale;
        let world_size = self.gen_settings.world_size;
        let room_size = self.gen_settings.room_size;

        // generate random noise
        let mut noise_map: HexGrid<HexGrid<f64>> =
            HexGrid::from_template(HexGrid::new(room_size), world_size);
        for (room, room_cord) in noise_map.iter_mut() {
            for (tile, tile_cord) in room.iter_mut() {
                let (x, y) = tile_to_worldspace(
                    tile_cord,
                    room_cord,
                    self.gen_settings.room_size,
                    noise_scale,
                );
                let value = world_gen::remap(-1.0, 1.0, 0.0, 1.0,  noise.get([x, y]));
                *tile = value;
            }
        }

        // put walls along the borders to rooms
        let mut room_edges_map: HexGrid<HexGrid<f64>> =
            HexGrid::from_template(HexGrid::new(room_size), world_size);
        for (room, room_cord) in room_edges_map.iter_mut() {
            for (tile, tile_cord) in room.iter_mut() {
                if tile_cord.magnitude() == room_size {
                    *tile = 1.0;
                }
            }
        }

        for (room, room_cord) in self.rooms.iter_mut() {
            for (tile, tile_cord) in room.tiles.iter_mut() {

                let noise_map_value = *noise_map.get(room_cord).unwrap().get(tile_cord).unwrap();
                let edges_map_value = *room_edges_map.get(room_cord).unwrap().get(tile_cord).unwrap();

                tile.wall = (noise_map_value + edges_map_value) > wall_threshhold;
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl From<WorldGenerationSettings> for World {
    fn from(settings: WorldGenerationSettings) -> Self {
        let mut world = World {
            rooms: HexGrid::from_template(Room::new(settings.room_size), settings.world_size),
            gen_settings: settings,
        };

        world.generate_walls();

        world
    }
}

#[derive(Serialize, Deserialize)]
pub struct WorldGenerationSettings {
    pub seed: i32,
    pub wall_threshold: f64,
    pub noise_scale: f64,

    pub room_size: i32,
    pub world_size: i32,
}

impl Default for WorldGenerationSettings {
    fn default() -> Self {
        Self {
            seed: 0,
            wall_threshold: 0.6,
            noise_scale: 0.08,
            room_size: 20,
            world_size: 20,
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

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Tile {
    pub wall: bool,
}

pub fn tile_to_worldspace(
    tile: Cordinate,
    room: Cordinate,
    room_size: i32,
    scale: f64,
) -> (f64, f64) {
    let (tile_x, tile_y) = tile.to_pixel_flat(scale);
    let (room_x, room_y) = room.to_pixel_point(scale * 3.0f64.sqrt() * room_size as f64);
    (tile_x + room_x, tile_y + room_y)
}
