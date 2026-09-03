use crate::consts;
use crate::coordinates::GridCoordinate;
use bevy::math::{I16Vec2, U16Vec2};
use consts::MapLogicLayers;

const PATH_START: u32 = 0xff00ff;
const PATH: u32 = 0xffff00;
const RESTRICTED: u32 = 0xff0000;
const PLACEABLE: u32 = 0x00ff00;
const WATER: u32 = 0x0000ff;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TileType {
    None,
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

fn rotate_right(direction: Direction) -> Direction {
    match direction {
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
    }
}

fn rotate_left(direction: Direction) -> Direction {
    match direction {
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Up => Direction::Left,
    }
}

fn is_path_tile(tile: TileType) -> bool {
    matches!(tile, TileType::Path | TileType::PathStart)
}

// EnemyPath

pub struct EnemyPath {
    path_corners: Vec<GridCoordinate>,
    path_length: u32,
}

impl EnemyPath {
    pub fn parse_from_map_tiles(map_tiles: &MapTiles) -> Option<EnemyPath> {
        let mut position = map_tiles.find_path_start()?;
        let mut direction = Direction::Right;

        let mut enemy_path = Self { path_corners: vec![position], path_length: 0 };

        println!("The path starts at: {:?}", position);
        println!("Start to follow path\n");

        loop {
            // Walk straight while there is a path
            while map_tiles.is_path_in_direction(position, direction) {
                position = map_tiles.get_position_in_direction(position, direction)?;

                enemy_path.path_length += 1;
                println!(
                    "Moved to {:?}, direction: {:?}, length: {:?}",
                    position, direction, enemy_path.path_length
                );

                // Safety against cyclic maps.
                if enemy_path.path_length > map_tiles.tiles.len() as u32 {
                    println!("Path parsing aborted: path is longer than the number of tiles.");
                    return None;
                }
            }

            println!("No path in front of: {:?}. Checking for corners.", position);

            // Don't add the same corner twice.
            if enemy_path.path_corners.last() != Some(&position) {
                enemy_path.path_corners.push(position);
            };

            // Try right
            let right = rotate_right(direction);

            if map_tiles.is_path_in_direction(position, right) {
                println!("Found path to the RIGHT. Turning {:?} -> {:?}", direction, right);

                direction = right;
                continue;
            }

            // Try left
            let left = rotate_left(direction);

            if map_tiles.is_path_in_direction(position, left) {
                println!("Found path to the LEFT. Turning {:?} -> {:?}", direction, left);

                direction = left;
                continue;
            }

            // No path except the direction we came from. Therefore, this is the end.
            println!("Found end at: {:?}", position);
            break;
        }

        println!("Path is {} tiles long", enemy_path.path_length);

        Some(enemy_path)
    }

    pub fn get_length(&self) -> u32 {
        self.path_length
    }

    pub fn corners(&self) -> &[GridCoordinate] {
        &self.path_corners
    }
}

// MapTiles

pub struct MapTiles {
    pub map_size: U16Vec2,
    pub tiles: Vec<TileType>,
}

impl MapTiles {
    pub fn load(logic_path: &str, map_size: U16Vec2) -> Option<Self> {
        let tiles = load_map_logic(logic_path, map_size)?;
        Some(Self { map_size, tiles })
    }

    pub fn map_size(&self) -> &U16Vec2 {
        &self.map_size
    }

    pub fn tiles(&self) -> &[TileType] {
        &self.tiles
    }

    pub fn get_tile_type(&self, coordinate: GridCoordinate) -> TileType {
        let Some(index) = coordinate.to_index(self.map_size) else {
            return TileType::None;
        };
        self.tiles[index]
    }

    pub fn is_tile_type(&self, coordinate: GridCoordinate, tile_type: TileType) -> bool {
        self.get_tile_type(coordinate) == tile_type
    }

    /// Prints the map in the same coordinate orientation as the game: bottom-left is (0,0).
    pub fn print_pixels(&self) {
        for y in (0..self.map_size.y).rev() {
            for x in 0..self.map_size.x {
                let coordinate = GridCoordinate::new(x, y);

                let tile = self.get_tile_type(coordinate);

                let symbol = match tile {
                    TileType::None => ' ',
                    TileType::PathStart => 'S',
                    TileType::Path => '#',
                    TileType::Restricted => 'X',
                    TileType::Placeable => '_',
                    TileType::Water => '~',
                };

                print!("{}", symbol);
            }
            println!();
        }
    }

    fn get_position_in_direction(
        &self, position: GridCoordinate, direction: Direction,
    ) -> Option<GridCoordinate> {
        let change = match direction {
            Direction::Right => I16Vec2::new(1, 0),
            Direction::Left => I16Vec2::new(-1, 0),
            Direction::Up => I16Vec2::new(0, 1),
            Direction::Down => I16Vec2::new(0, -1),
        };

        position.offset(change, self.map_size)
    }

    fn is_path_in_direction(&self, position: GridCoordinate, direction: Direction) -> bool {
        let Some(next) = self.get_position_in_direction(position, direction) else {
            return false;
        };

        is_path_tile(self.get_tile_type(next))
    }

    fn find_path_start(&self) -> Option<GridCoordinate> {
        for (index, tile) in self.tiles.iter().enumerate() {
            if *tile == TileType::PathStart {
                return GridCoordinate::from_index(index, self.map_size);
            }
        }
        None
    }
}

// GameMap

pub struct GameMap {
    pub map_tiles: MapTiles,
    pub enemy_path: EnemyPath,
}

impl GameMap {
    pub fn load(logic_layer: MapLogicLayers, map_size: U16Vec2) -> Option<Self> {
        let map_tiles = MapTiles::load(logic_layer.get_abs_path(), map_size)?;

        let enemy_path = EnemyPath::parse_from_map_tiles(&map_tiles)?;

        Some(Self { map_tiles, enemy_path })
    }

    pub fn map_tiles(&self) -> &MapTiles {
        &self.map_tiles
    }

    pub fn enemy_path(&self) -> &EnemyPath {
        &self.enemy_path
    }

    pub fn test_for_tile_type(&self, coordinate: GridCoordinate, tile_type: TileType) -> bool {
        self.map_tiles.is_tile_type(coordinate, tile_type)
    }

    pub fn return_tile_type(&self, coordinate: GridCoordinate) -> TileType {
        self.map_tiles.get_tile_type(coordinate)
    }
}

// PNG parsing

pub fn load_map_logic(path: &str, expected_size: U16Vec2) -> Option<Vec<TileType>> {
    let image = image::open(path).ok()?.to_rgb8();

    if image.width() != expected_size.x as u32 || image.height() != expected_size.y as u32 {
        println!(
            "Invalid map dimensions. Expected {}x{}, got {}x{}.",
            expected_size.x,
            expected_size.y,
            image.width(),
            image.height()
        );
        return None;
    }

    image
        .pixels()
        .map(|pixel| {
            let [r, g, b] = pixel.0;

            let color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            interpret_tile_from_rgb(color)
        })
        .collect()
}

pub fn load_map_raw(path: &str) -> Option<Vec<u32>> {
    let image = image::open(path).ok()?.to_rgb8();

    Some(
        image
            .pixels()
            .map(|pixel| {
                let [r, g, b] = pixel.0;

                ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
            })
            .collect(),
    )
}

fn interpret_tile_from_rgb(color: u32) -> Option<TileType> {
    match color {
        PATH_START => Some(TileType::PathStart),
        PATH => Some(TileType::Path),
        RESTRICTED => Some(TileType::Restricted),
        PLACEABLE => Some(TileType::Placeable),
        WATER => Some(TileType::Water),
        _ => {
            println!("Unknown map color: #{color:06X}");
            None
        },
    }
}
