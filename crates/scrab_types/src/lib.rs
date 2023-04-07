use hex_grid::{Cordinate, HexGrid};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub rooms: HexGrid<Room>,
    entities: Vec<Entity>,
    pub room_size: i32,
    pub world_size: i32,
    pub current_tick: u64,
    next_entity_id: u128,
}

impl World {
    pub fn new(world_size: i32, room_size: i32) -> Self {
        let room_template = Room::new(room_size);
        World {
            rooms: HexGrid::from_template(room_template.clone(), world_size),
            room_size,
            world_size,
            current_tick: 0,
            entities: Vec::new(),
            next_entity_id: 0,
        }
    }

    pub fn new_entity(&mut self, entity: EntityBuilder) -> Result<&mut Entity, EntityBuilderError> {
        self.entities.push(entity.build(self.next_entity_id)?);
        self.next_entity_id += 1;
        Ok(self.entities.last_mut().unwrap())
    }
}

pub type ScrabResult<T> = Result<T, ScrabError>;

#[derive(Error, Debug)]
pub enum ScrabError {
    #[error("Could not find entity with id {0:?}")]
    EntityNotFound(Position),
    #[error("The position {0:?} is invalid")]
    InvalidPosition(Position),
    #[error("This space is already occupied {0:?}")]
    SpaceOcupied(Position),
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
    tile: Cordinate,
}

#[derive(Serialize, Deserialize, Clone, )]
pub struct Entity {
    id: u128,
    pos: Position,
    owner: String,
    entity_type: EntityType,
}

impl Entity {
    pub fn pos(&self) -> Position {
        self.pos
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn entity_type(&self) -> &EntityType {
        &self.entity_type
    }

    pub fn id(&self) -> u128 {
        self.id
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum EntityType {
    Crab,
}

pub struct EntityBuilder {
    pos: Option<Position>,
    owner: Option<String>,
    entity_type: Option<EntityType>,
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self {
            pos: None,
            owner: None,
            entity_type: None,
        }
    }

    pub fn set_pos(&mut self, pos: Position) -> &mut Self {
        self.pos = Some(pos);
        self
    }

    pub fn set_owner(&mut self, owner: &str) -> &mut Self {
        self.owner = Some(owner.to_string());
        self
    }

    pub fn set_entity_type(&mut self, e_type: EntityType) -> &mut Self {
        self.entity_type = Some(e_type);
        self
    }

    pub(crate) fn build(self, id: u128) -> Result<Entity, EntityBuilderError> {
        if self.pos.is_none() || self.owner.is_none() || self.entity_type.is_none() {
            return Err(EntityBuilderError::FieldsNotSet);
        }

        Ok(Entity {
            id,
            pos: self.pos.unwrap(),
            owner: self.owner.unwrap(),
            entity_type: self.entity_type.unwrap(),
        })
    }
}

#[derive(Error, Debug)]
pub enum EntityBuilderError {
    #[error("Not all fields were set")]
    FieldsNotSet,
}
