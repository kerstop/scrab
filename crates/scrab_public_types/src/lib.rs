use hex_grid::Cordinate;
use schemars::JsonSchema;
use scrab_types::*;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct WorldManifest {
    pub rooms: Vec<RoomManifest>,
}

impl From<&World> for WorldManifest {
    fn from(world: &World) -> Self {
        let mut rooms: Vec<RoomManifest> = Vec::new();

        for room_cord in world.rooms.cordinates() {
            let (x, y) =
                room_cord.to_pixel_point(100.0 * 3.0f64.sqrt() * (world.room_size + 1) as f64);
            rooms.push(RoomManifest {
                name: room_cord.to_string(),
                screen_space_x: x,
                screen_space_y: y,
            })
        }

        WorldManifest { rooms }
    }
}

#[derive(Serialize, JsonSchema)]
pub struct RoomManifest {
    pub name: String,
    pub screen_space_x: f64,
    pub screen_space_y: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct PubRoom {
    pub tiles: Vec<PublicTile>,
    pub entities: Vec<PubEntity>,
}

impl From<&Room> for PubRoom {
    fn from(room: &Room) -> Self {
        let mut tiles = Vec::new();

        for cord in room.tiles.cordinates() {
            if let Some(tile) = room.tiles.get(cord) {
                tiles.push(PublicTile {
                    wall: tile.wall,
                    name: cord.to_string(),
                    cord: PubCord::from(cord),
                })
            }
        }

        let entities = room.entities.iter().map(|e| PubEntity::from(e)).collect();

        PubRoom { tiles, entities }
    }
}

#[derive(Serialize, JsonSchema)]
pub struct PublicTile {
    pub wall: bool,
    pub name: String,
    pub cord: PubCord,
}

#[derive(Serialize, JsonSchema)]
pub struct PubEntity {
    pub id: u128,
    pub pos: PubCord,
    pub ty: PubEntityType,
}

impl From<&Entity> for PubEntity {
    fn from(entity: &Entity) -> Self {
        PubEntity {
            id: entity.id(),
            pos: entity.pos().tile().into(),
            ty: entity.entity_type().into(),
        }
    }
}

#[derive(Serialize, JsonSchema)]
pub enum PubEntityType {
    Crab,
}

impl From<&EntityType> for PubEntityType {
    fn from(entity_type: &EntityType) -> Self {
        match entity_type {
            EntityType::Crab => PubEntityType::Crab,
        }
    }
}

#[derive(Serialize, JsonSchema)]
pub struct PubCord {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl From<Cordinate> for PubCord {
    fn from(value: Cordinate) -> Self {
        PubCord {
            q: value.q(),
            r: value.r(),
            s: value.s(),
        }
    }
}
