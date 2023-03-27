
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct PublicWorld {
    pub rooms: Vec<PublicRoomManifest>,
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

#[derive(Serialize, JsonSchema)]
pub struct PublicTile {
    pub wall: bool,
    pub name: String,
    pub x: f64,
    pub y: f64,
}
