use map_parsing::{load_map_logic, print_pixels, GameMap, MapTiles, TileType, Vector2D};

fn main() {
    let map_size = Vector2D{x: 32, y: 16};
    let tiles = load_map_logic(r"D:\Data D\commandline_defense\assets\maps\backrooms\logic_layer.png").expect("Could not load logic map");
    print_pixels(&tiles, map_size);
    
    let map_tiles = MapTiles{map_size, tiles };
    map_tiles.parse_enemy_path();
}
