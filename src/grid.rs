use bevy::color::Color;
use bevy::prelude::{default, Commands, Sprite, Transform, Vec2};
use crate::consts::{MAP_SIZE_TILES, TILE_SIZE};

pub fn spawn_grid(mut commands: Commands) {
    let width = MAP_SIZE_TILES[0] * TILE_SIZE;
    let height = MAP_SIZE_TILES[1] * TILE_SIZE;

    let line_thickness = 0.025;

    // Vertical lines
    for x in 0..=MAP_SIZE_TILES[0] {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(
                    line_thickness,
                    height as f32,
                )),
                ..default()
            },
            Transform::from_xyz(
                x as f32,
                0.0,
                10.0,
            ),
        ));
    }

    // Horizontal lines
    for y in 0..=MAP_SIZE_TILES[1] {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(
                    width as f32,
                    line_thickness,
                )),
                ..default()
            },
            Transform::from_xyz(
                0.0,
                y as f32,
                10.0,
            ),
        ));
    }
}