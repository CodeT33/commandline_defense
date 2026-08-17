use bevy_math::U16Vec2;
use map_parsing::GameMap;

fn main() {
    let map_size = U16Vec2::new(32, 16);
    let game_map = GameMap::load(r"assets\maps\backrooms\logic_layer.png", map_size)
        .expect("Could not load game map");

    game_map.map_tiles().print_pixels();
}
