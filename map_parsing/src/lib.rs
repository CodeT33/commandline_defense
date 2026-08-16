use image::{RgbImage};

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

    fn rotate_behind(current_direction: Direction) -> Direction {
        match current_direction {
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
        }
    }



#[derive(Clone, Copy, Debug)]
pub struct Vector2D {
    pub x: u16,
    pub y: u16,
}

pub struct EnemyPath {
    path_corners: Vec<Vector2D>,
    path_length: u32,
}

pub struct MapTiles {
    pub map_size: Vector2D,
    pub tiles: Vec<TileType>,
}

pub struct GameMap {
    pub map_tiles: MapTiles,
    pub enemy_path: EnemyPath,
}

pub fn get_index_from_map_position(map_position: Vector2D, map_size: Vector2D) -> usize {
    (map_position.y * map_size.x + map_position.x) as usize
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

fn is_path_tile(tile: &TileType) -> bool {
    matches!(tile, TileType::Path | TileType::PathStart)
}

impl MapTiles {
    fn is_path_in_direction(
        &self,
        position: Vector2D,
        direction: Direction,
    ) -> bool {
        let directed_position = match direction {
            Direction::Right => {
                if position.x + 1 >= self.map_size.x {
                    return false;
                }

                Vector2D {
                    x: position.x + 1,
                    y: position.y,
                }
            }

            Direction::Down => {
                if position.y + 1 >= self.map_size.y {
                    return false;
                }

                Vector2D {
                    x: position.x,
                    y: position.y + 1,
                }
            }

            Direction::Left => {
                if position.x == 0 {
                    return false;
                }

                Vector2D {
                    x: position.x - 1,
                    y: position.y,
                }
            }

            Direction::Up => {
                if position.y == 0 {
                    return false;
                }

                Vector2D {
                    x: position.x,
                    y: position.y - 1,
                }
            }
        };

        let index = get_index_from_map_position(
            directed_position,
            self.map_size,
        );

        self.tiles[index] == TileType::Path
    }

    fn move_in_direction(
        &self,
        position: Vector2D,
        direction: Direction,
    ) -> Vector2D {
        match direction {
            Direction::Right => Vector2D {
                x: position.x + 1,
                y: position.y,
            },

            Direction::Down => Vector2D {
                x: position.x,
                y: position.y + 1,
            },

            Direction::Left => Vector2D {
                x: position.x - 1,
                y: position.y,
            },

            Direction::Up => Vector2D {
                x: position.x,
                y: position.y - 1,
            },
        }
    }


    pub fn parse_enemy_path(&self) -> Option<EnemyPath> {
        let mut temp_position = self.find_path_start()?;
        let mut temp_direction = Direction::Right;

        let mut enemy_path = EnemyPath{path_corners: Vec::new(), path_length: 0};

        println!("The path starts at: {:?}", temp_position);

        println!("Start to follow path\n");

        for i in 0..20 {
            println!("Search for path at: {:?} {:?}", temp_position, temp_direction);
            while (self.is_path_in_direction(temp_position, temp_direction)) {
                temp_position = self.move_in_direction(temp_position, temp_direction);
                println!("Found it and moved to it");
                enemy_path.path_length += 1;
            }
            println!("Found corner: {:?}", temp_position);
            enemy_path.path_corners.push(temp_position);
            let direction_right = rotate_right(temp_direction);
            temp_direction = rotate_right(temp_direction);
        }

        println!("Path is {} tiles long", enemy_path.path_length);

        Some(enemy_path)

        /*
        while path in front
            move
            length += 1
        save position
        look for next turn
            rotate right
                test for path
            rotate behind
                test for path



        */

    }

    fn find_path_start(&self) -> Option<Vector2D> {
        for (index, tile) in self.tiles.iter().enumerate() {
            if *tile == TileType::PathStart {
                return Some(get_map_position_from_index(index, self.map_size));
            }
        }
        None
    }
}



