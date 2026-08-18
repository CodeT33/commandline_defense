use crate::consts;
use crate::consts::ui::grid::{GRID_LINE_THICKNESS, GRID_POSITION_COLOR};
use crate::consts::{MAP_SIZE_TILES, TILE_SIZE};
use bevy::prelude::{Commands, Sprite, Text2d, Transform, Vec2, Vec3, default};
use bevy::text::*;

pub fn get_letter_from_number(number: u16) -> char {
    match number {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        5 => 'F',
        6 => 'G',
        7 => 'H',
        8 => 'I',
        9 => 'J',
        10 => 'K',
        11 => 'L',
        12 => 'M',
        13 => 'N',
        14 => 'O',
        15 => 'P',
        16 => 'Q',
        17 => 'R',
        18 => 'S',
        19 => 'T',
        20 => 'U',
        21 => 'V',
        22 => 'W',
        23 => 'X',
        24 => 'Y',
        25 => 'Z',
        _ => '_',
    }
}

pub fn get_number_from_letter(letter: char) -> Option<u16> {
    match letter.to_ascii_uppercase() {
        'A' => Some(0),
        'B' => Some(1),
        'C' => Some(2),
        'D' => Some(3),
        'E' => Some(4),
        'F' => Some(5),
        'G' => Some(6),
        'H' => Some(7),
        'I' => Some(8),
        'J' => Some(9),
        'K' => Some(10),
        'L' => Some(11),
        'M' => Some(12),
        'N' => Some(13),
        'O' => Some(14),
        'P' => Some(15),
        'Q' => Some(16),
        'R' => Some(17),
        'S' => Some(18),
        'T' => Some(19),
        'U' => Some(20),
        'V' => Some(21),
        'W' => Some(22),
        'X' => Some(23),
        'Y' => Some(24),
        'Z' => Some(25),

        _ => None,
    }
}

fn draw_grid_positions(commands: &mut Commands) {
    for x in 0..MAP_SIZE_TILES[0] {
        for y in 0..MAP_SIZE_TILES[1] {
            let position = format!("{};{}", x, get_letter_from_number(y));

            commands.spawn((
                Text2d::new(position),
                TextFont {
                    font_size: FontSize::Px(consts::ui::grid::GRID_POSITION_FONT_SIZE),
                    ..default()
                },
                TextColor(GRID_POSITION_COLOR),
                Transform::from_xyz(x as f32 + 0.5, (MAP_SIZE_TILES[1] - y) as f32 - 0.5, 11.0)
                    .with_scale(Vec3::splat(0.025)),
            ));
        }
    }
}

pub fn draw_grid_overlay(commands: &mut Commands, show_grid_positions: bool) {
    let width = MAP_SIZE_TILES[0] * TILE_SIZE;
    let height = MAP_SIZE_TILES[1] * TILE_SIZE;

    // Vertical lines
    for x in 0..=MAP_SIZE_TILES[0] {
        commands.spawn((
            Sprite {
                color: consts::ui::grid::GRID_LINE_COLOR,
                custom_size: Some(Vec2::new(GRID_LINE_THICKNESS, height as f32)),
                ..default()
            },
            Transform::from_xyz(x as f32, 0.0, 10.0),
        ));
    }

    // Horizontal lines
    for y in 0..=MAP_SIZE_TILES[1] {
        commands.spawn((
            Sprite {
                color: consts::ui::grid::GRID_LINE_COLOR,
                custom_size: Some(Vec2::new(width as f32, GRID_LINE_THICKNESS)),
                ..default()
            },
            Transform::from_xyz(0.0, y as f32, 10.0),
        ));
    }

    if show_grid_positions {
        draw_grid_positions(commands);
    }
}
