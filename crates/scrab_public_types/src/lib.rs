
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct PublicWorld {
    pub rooms: Vec<String>,
}

#[derive(Serialize, JsonSchema)]
pub struct PublicRoom {
    pub tiles: Vec<PublicTile>,
}

#[derive(Serialize, JsonSchema)]
pub struct PublicTile {
    pub wall: bool,
    pub name: String,
    pub x: i32,
    pub y: i32,
}
