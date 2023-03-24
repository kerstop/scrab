
use serde::Serialize;

#[derive(Serialize)]
pub struct PublicWorld {
    pub rooms: Vec<String>,
}

#[derive(Serialize)]
pub struct PublicRoom {
    pub tiles: Vec<PublicTile>,
}

#[derive(Serialize)]
pub struct PublicTile {
    pub wall: bool,
    pub name: String,
    pub x: i32,
    pub y: i32,
}
