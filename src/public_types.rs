
use serde::Serialize;

#[derive(Serialize)]
pub struct PublicWorld {
    pub rooms: Vec<String>,
}

impl From<&crate::world::World> for PublicWorld {
    fn from(world: &crate::world::World) -> Self {
        let mut rooms = Vec::new();

        for room in world.rooms.cordinates() {
            rooms.push(room.to_string())
        }

        PublicWorld { rooms }
    }
}

#[derive(Serialize)]
pub struct PublicRoom {
    pub tiles: Vec<PublicTile>,
}

impl From<&crate::world::Room> for PublicRoom {
    fn from(room: &crate::world::Room) -> Self {
        let mut tiles = Vec::new();

        for cord in room.tiles.cordinates() {
            if let Some(tile) = room.get(&cord) {
                let (x, y) = cord.to_pixel(100.0);
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

#[derive(Serialize)]
pub struct PublicTile {
    pub wall: bool,
    pub name: String,
    pub x: i32,
    pub y: i32,
}

#[cfg(test)]
mod test {
    use codegen::cord;

    use super::PublicWorld;

    #[test]
    fn world_serialization() {
        let world = PublicWorld {
            rooms: vec![
                cord!(0, 0, 0).to_string(),
                cord!(1, -1, 0).to_string(),
                cord!(1, 0, -1).to_string(),
            ],
        };

        let ser_world = serde_json::to_string(&world).unwrap();
        let expected_format = "{\"rooms\":[\"[0,0,0]\",\"[1,-1,0]\",\"[1,0,-1]\"]}";
        assert_eq!(&ser_world, expected_format);
    }
}
