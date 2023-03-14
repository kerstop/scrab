struct World {
rooms: crate::hex_grid::HexGrid<Room>
}

struct Room {
tiles: crate::hex_grid::HexGrid<Tile>
}

struct Tile {
wall: bool,
}
