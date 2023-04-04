use std::collections::HashMap;

use hex_grid::{HexGrid, Cordinate};
use log::info;
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub rooms: HexGrid<Room>,
    entities: HashMap<Position, Vec<Entity>>,
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
            entities: HashMap::new(),
        }
    }

    pub fn insert_entity(&mut self, entity: Entity) -> ScrabResult<()> {
        let pos = entity.pos.clone();
        let owner = entity.owner.clone();

        if pos.room.magnitude() > self.world_size || pos.tile.magnitude() > self.room_size {
            return Err(ScrabError::InvalidPosition(pos))
        }

        match self.entities.insert( pos, vec![entity]) {
            Some(mut value) => self.entities.get_mut(&pos).unwrap().append(&mut value),
            None => (),
        }

        info!("{} inserted an entity", owner);
        Ok(())
    }

    pub fn remove_entity(&mut self, id: u128) -> ScrabResult<()> {
        for (_pos, list) in self.entities.iter_mut(){
            for i in 0..list.len() {
                if list.get(i).unwrap().id == id {
                    list.remove(i);
                    return Ok(())
                }
            }
        }
        Err(ScrabError::EntityNotFound(id))
    }
}

pub type ScrabResult<T> = Result<T, ScrabError>;

#[derive(Error, Debug)]
pub enum ScrabError {
    #[error("Could not find entity with id {0}")]
    EntityNotFound(u128),
    #[error("The position {0:?} is invalid")]
    InvalidPosition(Position),
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


#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    room: Cordinate,
    tile: Cordinate
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entity {
    id: u128,
    pos: Position,
    owner: String,
}