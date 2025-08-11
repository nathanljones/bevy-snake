
use bevy::color::palettes::css::{BLUE, GREEN};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct SnakeSegment;



#[derive(Component)]
#[require(Transform)]
struct Position(Vec2);
#[derive(Default)]
enum SnakeDirection {
    #[default]
    Up,
    Down,
    Left,
    Right,
}
#[derive(Resource, Default)]
struct CurrentSnakeDirection {
    snake_direction: SnakeDirection,
}
fn main() {
    App::new()
        .init_resource::<CurrentSnakeDirection>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_snake_head, spawn_snake_body))
        .add_systems(Update, (change_snake_direction, project_positions))
        //.add_systems(Update, change_speed)
        .add_systems(FixedUpdate,move_snake)
        .insert_resource(Time::<Fixed>::from_seconds(0.15))
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
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Position(Vec2::new(0., -20.)),
    ));
    commands.spawn((
        SnakeSegment,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Position(Vec2::new(0., -40.)),
    ));
}

fn change_snake_direction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut current_snake_direction: ResMut<CurrentSnakeDirection>,
){
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        current_snake_direction.snake_direction = SnakeDirection::Up;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        current_snake_direction.snake_direction = SnakeDirection::Down;
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
        current_snake_direction.snake_direction = SnakeDirection::Left;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        current_snake_direction.snake_direction = SnakeDirection::Right;
    }
}

fn move_snake(
    mut snake_head: Query<&mut Position, With<SnakeHead>>,
    mut snake_segments: Query<&mut Position, (With<SnakeSegment>,Without<SnakeHead>)>,
    current_snake_direction: Res<CurrentSnakeDirection>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) -> Result {
    let window = window_query.single()?;
    if let Ok(mut snake_head_position) = snake_head.single_mut() {
        let snake_head_pos = snake_head_position.0;
        match current_snake_direction.snake_direction {
            SnakeDirection::Up => {
                if snake_head_position.0.y > window.height() / 2.0 {
                    snake_head_position.0.y = -window.height() / 2.0;
                } else {
                    snake_head_position.0.y += 20.0;
                }
            }
            SnakeDirection::Down => {
                if snake_head_position.0.y < -window.height() / 2.0 {
                    snake_head_position.0.y = window.height() / 2.0;
                } else {
                    snake_head_position.0.y -= 20.0;
                }
            }
            SnakeDirection::Left => {
                if snake_head_position.0.x < -window.width() / 2.0 {
                    snake_head_position.0.x = window.width() / 2.0;
                } else {
                    snake_head_position.0.x -= 20.0
                };
            }
            SnakeDirection::Right => {
                if snake_head_position.0.x > window.width() / 2.0 {
                    snake_head_position.0.x = -window.width() / 2.0;
                } else {
                    snake_head_position.0.x += 20.0
                };
            }
        }

           let mut temp_pos = snake_head_pos;
            for mut snake_segment in &mut snake_segments.iter_mut() {
                let current_pos = snake_segment.0;
                snake_segment.0 = temp_pos;
                temp_pos = current_pos;
            }
    }
    Ok(())
}
fn project_positions(mut block_position: Query<(&mut Transform, &Position)>,) {
    for (mut transform, position) in &mut block_position {
        transform.translation = position.0.extend(0.);
    }
}
/*
fn change_speed(    keyboard_input: Res<ButtonInput<KeyCode>>,mut time: ResMut<Time<Fixed>>){
    if keyboard_input.pressed(KeyCode::Space) {
        time.set_timestep(Duration::from_secs(1));
    }
}
*/