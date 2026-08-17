use bevy_math::U16Vec2;
use image::RgbImage;

const PATH_START: u32 = 0xff00ff;
const PATH: u32 = 0xffff00;
const RESTRICTED: u32 = 0xff0000;
const PLACEABLE: u32 = 0x00ff00;
const WATER: u32 = 0x0000ff;

#[derive(PartialEq)]
pub enum TileType {
    PathStart,
    Path,
    Restricted,
    Placeable,
    Water,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn rotate_right(current_direction: Direction) -> Direction {
    match current_direction {
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
    }
}

fn rotate_left(current_direction: Direction) -> Direction {
    match current_direction {
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Up => Direction::Left,
    }
}

pub struct EnemyPath {
    path_corners: Vec<U16Vec2>,
    path_length: u32,
}

impl EnemyPath {
    pub fn get_length(&self) -> u32 {
        self.path_length
    }

    pub fn corners(&self) -> &[U16Vec2] {
        &self.path_corners
    }
}

pub struct MapTiles {
    pub map_size: U16Vec2,
    pub tiles: Vec<TileType>,
}

pub struct GameMap {
    pub map_tiles: MapTiles,
    pub enemy_path: EnemyPath,
}

pub fn get_index_from_map_position(map_position: U16Vec2, map_size: U16Vec2) -> usize {
    (map_position.y * map_size.x + map_position.x) as usize
}

pub fn get_map_position_from_index(index: usize, map_size: U16Vec2) -> U16Vec2 {
    U16Vec2::new(index as u16 % map_size.x, index as u16 / map_size.x)
}

pub fn load_map_logic(path: &str) -> Option<Vec<TileType>> {
    let image: RgbImage = image::open(path).ok()?.to_rgb8();

    let tiles = image
        .pixels()
        .map(|pixel| {
            let [r, g, b] = pixel.0;

            let color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);

            interpret_tile_from_rgb(color)
        })
        .collect();

    tiles
}

pub fn load_map_raw(path: &str) -> Option<Vec<u32>> {
    let image = image::open(path).ok()?.to_rgb8();

    let pixels: Vec<u32> = image
        .pixels()
        .map(|pixel| {
            let [r, g, b] = pixel.0;

            ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
        })
        .collect();

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

pub fn print_pixels(pixels: &[TileType], map_size: U16Vec2) {
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

fn is_path_tile(tile: &TileType) -> bool {
    matches!(tile, TileType::Path | TileType::PathStart)
}

impl MapTiles {
    fn is_path_in_direction(&self, position: U16Vec2, direction: Direction) -> bool {
        let directed_position = match direction {
            Direction::Right => {
                if position.x + 1 >= self.map_size.x {
                    return false;
                }

                U16Vec2::new(position.x + 1, position.y)
            },

            Direction::Down => {
                if position.y + 1 >= self.map_size.y {
                    return false;
                }

                U16Vec2::new(position.x, position.y + 1)
            },

            Direction::Left => {
                if position.x == 0 {
                    return false;
                }

                U16Vec2::new(position.x - 1, position.y)
            },

            Direction::Up => {
                if position.y == 0 {
                    return false;
                }

                U16Vec2::new(position.x, position.y - 1)
            },
        };

        let index = get_index_from_map_position(directed_position, self.map_size);

        is_path_tile(&self.tiles[index])
    }

    fn move_in_direction(&self, position: U16Vec2, direction: Direction) -> U16Vec2 {
        match direction {
            Direction::Right => U16Vec2::new(position.x + 1, position.y),

            Direction::Down => U16Vec2::new(position.x, position.y + 1),

            Direction::Left => U16Vec2::new(position.x - 1, position.y),

            Direction::Up => U16Vec2::new(position.x, position.y - 1),
        }
    }

    pub fn parse_enemy_path(&self) -> Option<EnemyPath> {
        let mut temp_position = self.find_path_start()?;
        let mut temp_direction = Direction::Right;

        let mut enemy_path = EnemyPath { path_corners: vec![temp_position], path_length: 0 };

        println!("The path starts at: {:?}", temp_position);
        println!("Start to follow path\n");

        loop {
            while self.is_path_in_direction(temp_position, temp_direction) {
                temp_position = self.move_in_direction(temp_position, temp_direction);
                enemy_path.path_length += 1;
                println!(
                    "Moved to {:?}, {:?}, {:?}",
                    temp_position, temp_direction, enemy_path.path_length
                );
            }
            println!("No path in front of: {:?}. Checking for corners.", temp_position);
            enemy_path.path_corners.push(temp_position);

            let right_direction = rotate_right(temp_direction);

            if self.is_path_in_direction(temp_position, right_direction) {
                println!(
                    "Found path to the RIGHT. Turning from {:?} to {:?}.",
                    temp_direction, right_direction
                );

                temp_direction = right_direction;
                continue;
            }

            let left_direction = rotate_left(temp_direction);

            if self.is_path_in_direction(temp_position, left_direction) {
                println!(
                    "Found path to the LEFT. Turning from {:?} to {:?}.",
                    temp_direction, left_direction
                );

                temp_direction = left_direction;
                continue;
            }

            println!("Found end at: {:?}", temp_position);
            break;
        }

        println!("Path is {} tiles long", enemy_path.path_length);

        Some(enemy_path)

        /*
        while path in front
            move
            length += 1
        save position of corner
        rotate right
            test for path
                -> start again in while with the actual direction
        rotate behind
            test for path
                -> start again in while with the actual direction
        if nothing except the direction we came from can be found, the end is reached
        */
    }

    fn find_path_start(&self) -> Option<U16Vec2> {
        for (index, tile) in self.tiles.iter().enumerate() {
            if *tile == TileType::PathStart {
                return Some(get_map_position_from_index(index, self.map_size));
            }
        }
        None
    }
}
