use bevy::math::{I16Vec2, U16Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Logical coordinate of a tile in the game grid.
///
/// Coordinate system:
///
/// (0, 0) = bottom-left
///
/// IMPORTANT:
/// PNG/image coordinates are NOT the same.
/// The conversion to a Vec index handles the vertical flip.
pub struct GridCoordinate {
    pub position: U16Vec2,
}

impl GridCoordinate {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { position: U16Vec2::new(x, y) }
    }

    pub const fn from_u16vec2(position: U16Vec2) -> Self {
        Self { position }
    }

    pub const fn x(&self) -> u16 {
        self.position.x
    }

    pub const fn y(&self) -> u16 {
        self.position.y
    }

    /// Checks whether this position is inside the given map.
    pub fn is_on_map(&self, map_size: U16Vec2) -> bool {
        self.position.x < map_size.x && self.position.y < map_size.y
    }

    /// Checks whether a position is inside the given map.
    pub fn is_position_on_map(position: U16Vec2, map_size: U16Vec2) -> bool {
        position.x < map_size.x && position.y < map_size.y
    }

    /// Converts the logical bottom-left coordinate into
    /// the index used by the image/tile Vec.
    ///
    /// The Vec is stored in PNG order:
    ///
    /// index 0 = top-left pixel
    ///
    /// Therefore, Y needs to be inverted here.
    pub fn to_index(&self, map_size: U16Vec2) -> Option<usize> {
        if !self.is_on_map(map_size) {
            return None;
        }

        let x = self.position.x as usize;
        let y = self.position.y as usize;

        let width = map_size.x as usize;
        let height = map_size.y as usize;

        Some((height - 1 - y) * width + x)
    }

    /// Converts a Vec/image index back into a logical bottom-left GridCoordinate.
    pub fn from_index(index: usize, map_size: U16Vec2) -> Option<Self> {
        let width = map_size.x as usize;
        let height = map_size.y as usize;

        if width == 0 || height == 0 {
            return None;
        }

        if index >= width * height {
            return None;
        }

        let image_y = index / width;
        let x = index % width;
        let y = height - 1 - image_y;

        Some(Self::new(x as u16, y as u16))
    }

    /// Returns a neighboring coordinate.
    /// Useful for pathfinding and other grid operations.
    pub fn offset(&self, offset: I16Vec2, map_size: U16Vec2) -> Option<Self> {
        let position = self.position.checked_add_signed(offset)?;

        Self::is_position_on_map(position, map_size).then_some(Self::from_u16vec2(position))
    }
}
