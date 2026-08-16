pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/**
`#00ffff` (aqua) path start
`#ff00ff` (pink) path end
`#ffff00` (yellow) path

`#ff0000` (red) restricted
`#00ff00` (green) placeable
`#0000ff` (blue) water
*/

enum TileType {
    PathStart,
    PathEnd,
    Path,
    Restricted,
    Placeable,
    Water,
}

struct Vector2D {
    x: u16,
    y: u16,
}

struct Path {
    path_corners: Vec<Vector2D>,
}

struct GameMap {
    map_size: Vector2D,
    tiles: Vec<TileType>,
    path: Path,
}

fn get_index_from_map_position(map_position: Vector2D) -> usize {
    (map_position.x * map_position.y + map_position.x) as usize
}

fn get_map_position_from_index(index: usize, map_size: Vector2D) -> Vector2D {
    Vector2D{x: index as u16 % map_size.x, y: index as u16 / map_size.x}
}
