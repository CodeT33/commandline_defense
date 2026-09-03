use crate::components::{GridLine, GridOverlay, GridPositionLabel};
use crate::consts::ui::grid::GRID_LINE_THICKNESS;
use crate::consts::{self};
use crate::coordinates::GridCoordinate;
use crate::game_cli::command_line_state_management::PreviewCommand;
use crate::game_map::map_logic_parsing::TileType;
use crate::resources::{CommandState, MapResource};
use bevy::prelude::{
    Commands, Query, Res, Sprite, Text2d, Transform, Vec2, Vec3, Visibility, With, default,
};
use bevy::text::*;

pub fn get_letter_from_number(number: u16) -> char {
    if number < 26 { (b'A' + number as u8) as char } else { '_' }
}

pub fn get_number_from_letter(letter: char) -> Option<u16> {
    let letter = letter.to_ascii_uppercase();
    if letter.is_ascii_uppercase() { Some((letter as u8 - b'A') as u16) } else { None }
}

pub fn spawn_grid_positions(commands: &mut Commands, map_resource: &MapResource) {
    let meta_position_text_font: TextFont = TextFont {
        font_size: FontSize::Px(consts::ui::grid::GRID_META_POSITION.font_size),
        weight: consts::ui::grid::GRID_META_POSITION.font_weight,
        ..default()
    };
    let map_size = map_resource.0.map_tiles.map_size;

    for x in 0..map_size.x {
        commands.spawn((
            Text2d::new(x.to_string()),
            meta_position_text_font.clone(),
            TextColor(consts::ui::grid::GRID_META_POSITION.color),
            Transform::from_xyz(
                x as f32 + 0.5,
                map_size.y as f32 + 0.5,
                consts::rendering_layers::GRID_LABEL,
            )
            .with_scale(Vec3::splat(0.025)),
            GridPositionLabel,
            GridOverlay,
        ));
    }

    for y in 0..map_size.y {
        commands.spawn((
            Text2d::new(get_letter_from_number(y)),
            meta_position_text_font.clone(),
            TextColor(consts::ui::grid::GRID_META_POSITION.color),
            Transform::from_xyz(-0.5, y as f32 + 0.5, consts::rendering_layers::GRID_LABEL)
                .with_scale(Vec3::splat(0.025)),
            GridPositionLabel,
            GridOverlay,
        ));
    }

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let coordinate = GridCoordinate::new(x, y);
            let position = format!("{}{}", get_letter_from_number(y), x);

            let tile_type: TileType = map_resource.0.return_tile_type(coordinate);
            let text_color = match tile_type {
                TileType::None => consts::ui::grid::GRID_POSITION_TILE_COLORS.none,
                TileType::PathStart => consts::ui::grid::GRID_POSITION_TILE_COLORS.path_start,
                TileType::Path => consts::ui::grid::GRID_POSITION_TILE_COLORS.path,
                TileType::Restricted => consts::ui::grid::GRID_POSITION_TILE_COLORS.restricted,
                TileType::Placeable => consts::ui::grid::GRID_POSITION_TILE_COLORS.placeable,
                TileType::Water => consts::ui::grid::GRID_POSITION_TILE_COLORS.water,
            };

            commands.spawn((
                Text2d::new(position),
                TextFont {
                    font_size: FontSize::Px(consts::ui::grid::GRID_POSITION.font_size),
                    weight: consts::ui::grid::GRID_POSITION.font_weight,
                    ..default()
                },
                TextColor(text_color),
                Transform::from_xyz(
                    x as f32 + 0.5,
                    y as f32 + 0.5,
                    consts::rendering_layers::GRID_LABEL,
                )
                .with_scale(Vec3::splat(0.025)),
                GridPositionLabel,
                GridOverlay,
            ));
        }
    }
}

pub fn spawn_grid(commands: &mut Commands, map_resource: &MapResource) {
    let width = map_resource.0.map_tiles.map_size.x;
    let height = map_resource.0.map_tiles.map_size.y;

    // Vertical lines
    for x in 0..=width {
        commands.spawn((
            Sprite {
                color: consts::ui::grid::GRID_LINE_COLOR,
                custom_size: Some(Vec2::new(GRID_LINE_THICKNESS, height as f32)),
                ..default()
            },
            Transform::from_xyz(x as f32, (height / 2) as f32, consts::rendering_layers::GRID),
            GridLine,
            GridOverlay,
        ));
    }

    // Horizontal lines
    for y in 0..=height {
        commands.spawn((
            Sprite {
                color: consts::ui::grid::GRID_LINE_COLOR,
                custom_size: Some(Vec2::new(width as f32, GRID_LINE_THICKNESS)),
                ..default()
            },
            Transform::from_xyz((width / 2) as f32, y as f32, consts::rendering_layers::GRID),
            GridLine,
            GridOverlay,
        ));
    }
}

pub fn update_grid_preview(
    command_state: Res<CommandState>, mut grid_overlay: Query<&mut Visibility, With<GridOverlay>>,
) {
    let visible = matches!(command_state.preview, PreviewCommand::ShowGrid)
        || matches!(command_state.preview, PreviewCommand::HighlightTile { .. });

    for mut visibility in &mut grid_overlay {
        *visibility = if visible { Visibility::Visible } else { Visibility::Hidden };
    }
}

pub fn spawn_contrast_overlay(commands: &mut Commands, map_resource: &MapResource) {
    let map_size = map_resource.0.map_tiles.map_size;

    commands.spawn((
        Sprite {
            color: consts::ui::grid::GRID_CONTRAST_COLOR,
            custom_size: Option::from(Vec2::new(map_size.x as f32, map_size.y as f32)),
            ..default()
        },
        Transform::from_xyz(
            (map_size.x / 2) as f32,
            (map_size.y / 2) as f32,
            consts::rendering_layers::CONTRAST,
        ),
        GridOverlay,
        Visibility::Hidden,
    ));
}
