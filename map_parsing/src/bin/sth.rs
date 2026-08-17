use bevy_math::U16Vec2;
use map_parsing::{MapTiles, load_map_logic, print_pixels};

fn main() {
    let map_size = U16Vec2::new(32, 16);
    let tiles =
        load_map_logic(r"assets\maps\backrooms\logic_layer.png").expect("Could not load logic map");
    print_pixels(&tiles, map_size);

    let map_tiles = MapTiles { map_size, tiles };
    map_tiles.parse_enemy_path();
}
