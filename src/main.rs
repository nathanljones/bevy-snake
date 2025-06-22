use bevy::color::palettes::css::{BLUE, GREEN};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct SnakeSegment;

#[derive(Default, Component)]
struct SnakeSegments(Vec<Entity>);


#[derive(Component)]
#[require(Transform)]
struct Position(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_snake_head, spawn_snake_body))
        .add_systems(Update, (move_snake, project_positions))
        .run();
}
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn spawn_snake_head(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Rectangle::new(20.0, 20.0);
    let colour = Color::Srgba(BLUE);
    let mesh = meshes.add(shape);
    let material = materials.add(colour);
    commands.spawn((
        SnakeHead,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Position(Vec2::new(0., 0.)),
    ));
}
fn spawn_snake_body(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Rectangle::new(20.0, 20.0);
    let colour = Color::Srgba(GREEN);
    let mesh = meshes.add(shape);
    let material = materials.add(colour);
    commands.spawn((
        SnakeSegment,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Position(Vec2::new(0., -20.)),
    ));
}

fn move_snake(
    mut snake_head: Query<&mut Position, (With<SnakeHead>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) -> Result {
    let window = window_query.single()?;
    if let Ok(mut snake_head_position) = snake_head.single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            if snake_head_position.0.y > window.height() / 2.0 {
                snake_head_position.0.y = -1.0 * window.height() / 2.0;
            } else {
                snake_head_position.0.y += 10.0;
            }
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            if snake_head_position.0.y < -1.0 * window.height() / 2.0 {
                snake_head_position.0.y = window.height() / 2.0;
            } else {
                snake_head_position.0.y -= 10.0;
            }
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            if snake_head_position.0.x < -1.0 * window.width() / 2.0 {
                snake_head_position.0.x = window.width() / 2.0;
            } else {
                snake_head_position.0.x -= 10.0
            };
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            if snake_head_position.0.x > window.width() / 2.0 {
                snake_head_position.0.x = -1.0 * window.width() / 2.0;
            } else {
                snake_head_position.0.x += 10.0
            };
        }else{
            snake_head_position.0.x += 0.0;
            snake_head_position.0.y += 0.0;
        }
    }
    Ok(())
}
fn project_positions(mut block_position: Query<(&mut Transform, &Position)>,) {
    for (mut transform, position) in &mut block_position {
        transform.translation = position.0.extend(0.);
    }
}
