use bevy::math::{I16Vec2, U16Vec2};
use std::ops::Deref;

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
pub(crate) struct GridCoordinate(pub(crate) U16Vec2);

impl Deref for GridCoordinate {
    type Target = U16Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GridCoordinate {
    pub(crate) const fn new(x: u16, y: u16) -> Self {
        Self(U16Vec2::new(x, y))
    }

    pub(crate) const fn from_u16vec2(position: U16Vec2) -> Self {
        Self(position)
    }

    /// Checks whether this position is inside the given map.
    pub(crate) fn is_on_map(&self, map_size: U16Vec2) -> bool {
        self.x < map_size.x && self.y < map_size.y
    }

    /// Checks whether a position is inside the given map.
    pub(crate) fn is_position_on_map(position: U16Vec2, map_size: U16Vec2) -> bool {
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
    pub(crate) fn to_index(&self, map_size: U16Vec2) -> Option<usize> {
        if !self.is_on_map(map_size) {
            return None;
        }

        let x = self.x as usize;
        let y = self.y as usize;

        let width = map_size.x as usize;
        let height = map_size.y as usize;

        Some((height - 1 - y) * width + x)
    }

    /// Converts a Vec/image index back into a logical bottom-left GridCoordinate.
    pub(crate) fn from_index(index: usize, map_size: U16Vec2) -> Option<Self> {
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
    pub(crate) fn offset(&self, offset: I16Vec2, map_size: U16Vec2) -> Option<Self> {
        let position = self.checked_add_signed(offset)?;

        Self::is_position_on_map(position, map_size).then_some(Self::from_u16vec2(position))
    }
}
