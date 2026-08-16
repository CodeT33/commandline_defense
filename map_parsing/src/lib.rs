use image::{RgbImage};

const PATH_START: u32 = 0xff00ff;
const PATH: u32 = 0xffff00;
const RESTRICTED: u32 = 0xff0000;
const PLACEABLE: u32 = 0x00ff00;
const WATER: u32 = 0x0000ff;

pub enum TileType {
    PathStart,
    Path,
    Restricted,
    Placeable,
    Water,
}

pub struct Vector2D {
    pub x: u16,
    pub y: u16,
}

pub struct EnemyPath {
    path_corners: Vec<Vector2D>,
}

pub struct GameMap {
    map_size: Vector2D,
    tiles: Vec<TileType>,
    path: EnemyPath,
}

pub fn get_index_from_map_position(map_position: Vector2D) -> usize {
    (map_position.x * map_position.y + map_position.x) as usize
}

pub fn get_map_position_from_index(index: usize, map_size: Vector2D) -> Vector2D {
    Vector2D{x: index as u16 % map_size.x, y: index as u16 / map_size.x}
}

pub fn load_map_logic(path: &str) -> Option<Vec<TileType>>{
    let image: RgbImage = image::open(path).ok()?.to_rgb8();

    let tiles = image
        .pixels()
        .map(|pixel| {
            let [r, g, b] = pixel.0;

            let color = ((r as u32) << 16)
                | ((g as u32) << 8)
                | (b as u32);

            interpret_tile_from_rgb(color)
        })
        .collect();

    tiles
}

pub fn load_map_raw(path: &str) -> Option<Vec<u32>> {
    let image = image::open(path).ok()?.to_rgb8();

    let pixels: Vec<u32> = image.pixels().map(
        |pixel| {
            let [r, g, b] = pixel.0;

            ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
        }
    ).collect();

    Some(pixels)
}

fn interpret_tile_from_rgb(color: u32) -> Option<TileType> {
    Some(match color {
        PATH_START => TileType::PathStart,
        PATH => TileType::Path,
        RESTRICTED => TileType::Restricted,
        PLACEABLE => TileType::Placeable,
        WATER => TileType::Water,
        _ => None?,
    })
}

pub fn print_pixels(pixels: &[TileType], map_size: Vector2D) {
    for y in 0..map_size.y {
        for x in 0..map_size.x {
            let index = (y * map_size.x + x) as usize;

            let symbol = match pixels[index] {
                TileType::PathStart => 'S',
                TileType::Path => '#',
                TileType::Restricted => 'X',
                TileType::Placeable => '_',
                TileType::Water => '~',
            };

            print!("{symbol}");
        }
        println!();
    }
}