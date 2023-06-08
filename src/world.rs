use noise::{NoiseFn, Perlin};
use serde::{Deserialize, Serialize};

use hex_grid::{Cordinate, HexGrid};
use scrab_types::*;

fn generate_walls(world: &mut World, gen_settings: &WorldGenerationSettings) {
    let wall_threshhold = gen_settings.wall_threshold;
    let noise = Perlin::new(gen_settings.seed as u32);
    let noise_scale = gen_settings.noise_scale;
    let world_size = gen_settings.world_size;
    let room_size = gen_settings.room_size;

    // generate random noise
    let mut noise_map: HexGrid<HexGrid<f64>> =
        HexGrid::from_template(HexGrid::new(room_size), world_size);
    for (room, room_cord) in noise_map.iter_mut() {
        for (tile, tile_cord) in room.iter_mut() {
            let (x, y) =
                tile_to_worldspace(tile_cord, room_cord, gen_settings.room_size, noise_scale);
            let value = remap(-1.0, 1.0, 0.0, 1.0, noise.get([x, y]));
            *tile = value;
        }
    }

    // put walls along the borders to rooms
    let mut room_edges_map: HexGrid<HexGrid<f64>> =
        HexGrid::from_template(HexGrid::new(room_size), world_size);
    for (room, _room_cord) in room_edges_map.iter_mut() {
        for (tile, tile_cord) in room.iter_mut() {
            if tile_cord.magnitude() == room_size {
                *tile = 1.0;
            }
        }
    }

    for (room, room_cord) in world.rooms.iter_mut() {
        for (tile, tile_cord) in room.tiles.iter_mut() {
            let noise_map_value = *noise_map.get(room_cord).unwrap().get(tile_cord).unwrap();
            let edges_map_value = *room_edges_map
                .get(room_cord)
                .unwrap()
                .get(tile_cord)
                .unwrap();

            tile.is_wall = (noise_map_value + edges_map_value) > wall_threshhold;
        }
    }
}

impl From<WorldGenerationSettings> for World {
    fn from(settings: WorldGenerationSettings) -> Self {
        let mut world = World::new(settings.world_size, settings.room_size);

        generate_walls(&mut world, &settings);

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

fn remap(low1: f64, high1: f64, low2: f64, high2: f64, value: f64) -> f64 {
    low2 + (value - low1) * (high2 - low2) / (high1 - low1)
}