use bevy::input_focus::{
    AutoFocus, InputFocus,
    tab_navigation::{TabGroup, TabIndex, TabNavigationPlugin},
};
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle};
use bevy::window::{PresentMode, PrimaryWindow};
use rand::RngExt;

const RECT_SIZE: Vec2 = Vec2::new(140.0, 80.0);

#[derive(Component)]
struct Jitter;

pub(crate) fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2d game".into(),
                resolution: (800, 450).into(),
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TabNavigationPlugin)
        .insert_resource(Time::<Fixed>::from_hz(144.0))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, jitter_rectangle)
        .add_systems(Update, (pan_camera, submit_text))
        .run();
}

fn setup(
    mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera2d, IsDefaultUiCamera));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.85, 0.20, 0.25))),
        Transform::from_xyz(-120.0, 0.0, 0.0),
    ));

    commands.spawn((
        Jitter,
        Sprite {
            image: asset_server.load("sprites/uzsg4bc3e2mg1.png"),
            custom_size: Some(RECT_SIZE),
            ..default()
        },
        Transform::from_xyz(140.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_4))
            .with_scale(Vec3::splat(1.0)),
    ));

    spawn_text_input(&mut commands);
}

fn spawn_text_input(commands: &mut Commands) {
    commands.spawn((
        Text::new("click the box, type, Enter to print"),
        TextFont { font_size: FontSize::Px(16.0), ..default() },
        Node { ..default() },
    ));

    commands.spawn((
        Node {
            padding: px(8).all(),
            border: px(2).all(),
            align_items: AlignItems::Center,
            ..default()
        },
        EditableText { visible_width: Some(16.0), allow_newlines: false, ..default() },
        TextFont { font_size: FontSize::Px(24.0), ..default() },
        TextColor(Color::WHITE),
        TextCursorStyle::default(),
        BackgroundColor::default(),
        TabGroup::new(0),
        TabIndex(0),
        AutoFocus,
    ));
}

fn submit_text(
    focus: Res<InputFocus>, keys: Res<ButtonInput<KeyCode>>, mut inputs: Query<&mut EditableText>,
) {
    if !keys.just_pressed(KeyCode::Enter) {
        return;
    }
    let Some(entity) = focus.get() else {
        return;
    };
    let Ok(mut input) = inputs.get_mut(entity) else {
        return;
    };

    println!("{}", input.value());
    input.clear();
}

fn jitter_rectangle(
    mut q: Query<&mut Transform, With<Jitter>>, windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let half = Vec2::new(window.width(), window.height()) * 0.5 - RECT_SIZE * 0.5;
    let mut rng = rand::rng();

    for mut tf in &mut q {
        tf.translation.x = rng.random_range(-half.x..half.x);
        tf.translation.y = rng.random_range(-half.y..half.y);
    }
}

fn pan_camera(
    time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, focus: Res<InputFocus>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    if focus.get().is_some() {
        return;
    }

    let Ok(mut tf) = camera.single_mut() else {
        return;
    };

    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    if let Some(dir) = dir.try_normalize() {
        tf.translation += (dir * 400.0 * time.delta_secs()).extend(0.0);
    }
}
