use scrab_types::*;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct PublicWorld {
    pub rooms: Vec<PublicRoomManifest>,
}

impl From<&World> for PublicWorld {
    fn from(world: &World) -> Self {
        let mut rooms: Vec<PublicRoomManifest> = Vec::new();

        for room_cord in world.rooms.cordinates() {
            let (x, y) = room_cord
                .to_pixel_point(100.0 * 3.0f64.sqrt() * (world.room_size + 1) as f64);
            rooms.push(PublicRoomManifest {
                name: room_cord.to_string(),
                screen_space_x: x,
                screen_space_y: y,
            })
        }

        PublicWorld { rooms }
    }
}


#[derive(Serialize, JsonSchema)]
pub struct PublicRoomManifest {
    pub name: String,
    pub screen_space_x: f64,
    pub screen_space_y: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct PublicRoom {
    pub tiles: Vec<PublicTile>,
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

#[derive(Serialize, JsonSchema)]
pub struct PublicTile {
    pub wall: bool,
    pub name: String,
    pub x: f64,
    pub y: f64,
}
