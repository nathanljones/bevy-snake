use bevy::color::palettes::css::BLUE;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]

struct Block;

#[derive(Component)]
#[require(Transform)]
struct Position(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_block))
        .add_systems(Update, (move_block, project_positions))
        .run();
}
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn spawn_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Rectangle::new(20.0, 20.0);
    let colour = Color::Srgba(BLUE);
    let mesh = meshes.add(shape);
    let material = materials.add(colour);
    commands.spawn((
        Block,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Position(Vec2::new(0., 0.)),
    ));
}

fn move_block(
    mut block: Query<&mut Position, With<Block>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single().unwrap();
    if let Ok(mut position) = block.single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            if position.0.y > window.height() / 2.0 {
                position.0.y = -1.0 * window.height() / 2.0;
            } else {
                position.0.y += 10.0;
            }
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            if position.0.y < -1.0 * window.height() / 2.0 {
                position.0.y = window.height() / 2.0;
            } else {
                position.0.y -= 10.0;
            }
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            if position.0.x < -1.0 * window.width() / 2.0 {
                position.0.x = window.width() / 2.0;
            } else {
                position.0.x -= 10.0
            };
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            if position.0.x > window.width() / 2.0 {
                position.0.x = -1.0 * window.width() / 2.0;
            } else {
                position.0.x += 10.0
            };
        }
    }
}
fn project_positions(mut block_position: Query<(&mut Transform, &Position), With<Block>>) {
    for (mut transform, position) in &mut block_position {
        transform.translation = position.0.extend(0.);
    }
}
