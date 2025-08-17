use bevy::color::palettes::css::{BLUE, GREEN, LIGHT_GREEN, RED, YELLOW};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::Rng;
use std::time::Duration;

#[derive(Resource)]
struct GameBoard {
    width: u32,
    height: u32,
    cell_size: f32,
}
#[derive(Component)]
#[require(Sprite)]
struct SnakeHead;

#[derive(Component)]
#[require(Sprite)]
struct Apple;

#[derive(Component)]
#[require(Sprite)]
struct SnakeSegment;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct GridLocation(Vec2);
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
#[derive(Default, States, PartialEq, Debug, Clone, Eq, Hash)]
enum FoodState {
    #[default]
    NeedsSpawning,
    OnTheBoard,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CurrentSnakeDirection>()
        .init_state::<FoodState>()
        .add_systems(
            Startup,
            (
                spawn_camera,
                spawn_snake_head,
                spawn_snake_body,
                setup_board,
            ),
        )
        .add_systems(
            Update,
            (
                change_snake_direction,
                project_positions,
                draw_board,
                project_board,
            ),
        )
        //.add_systems(Update, change_speed)
        .add_systems(
            Update,
            move_snake.run_if(on_timer(Duration::from_millis(150))),
        )
        .add_systems(
            Update,
            spawn_apple.run_if(in_state(FoodState::NeedsSpawning)),
        )
        .insert_resource(Time::<Fixed>::from_seconds(2.0))
        .run();
}
fn setup_board(mut commands: Commands) {
    let board = GameBoard {
        width: 10,
        height: 10,
        cell_size: 20.0,
    };
    commands.insert_resource(board);
}
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn spawn_snake_head(mut commands: Commands) {
    let colour = Color::Srgba(BLUE);
    commands.spawn((
        SnakeHead,
        GridLocation(Vec2::new(5., 5.)),
        Sprite {
            color: colour,
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
    ));
}
fn spawn_snake_body(mut commands: Commands) {
    let colour = Color::Srgba(YELLOW);
    commands.spawn((
        SnakeSegment,
        GridLocation(Vec2::new(5., -4.)),
        Sprite {
            color: colour,
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
    ));

    commands.spawn((
        SnakeSegment,
        GridLocation(Vec2::new(5., -3.)),
        Sprite {
            color: colour,
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
    ));
}

fn change_snake_direction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut current_snake_direction: ResMut<CurrentSnakeDirection>,
) {
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
    mut snake_head: Query<&mut GridLocation, With<SnakeHead>>,
    mut snake_segments: Query<&mut GridLocation, (With<SnakeSegment>, Without<SnakeHead>)>,
    current_snake_direction: Res<CurrentSnakeDirection>,
) {
    if let Ok(mut snake_head_position) = snake_head.single_mut() {
        let snake_head_pos = snake_head_position.0;
        match current_snake_direction.snake_direction {
            SnakeDirection::Up => snake_head_position.0.y += 1.0,
            SnakeDirection::Down => snake_head_position.0.y -= 1.0,
            SnakeDirection::Left => snake_head_position.0.x -= 1.0,
            SnakeDirection::Right => snake_head_position.0.x += 1.0,
        }

        let mut temp_pos = snake_head_pos;
        for mut snake_segment in &mut snake_segments.iter_mut() {
            std::mem::swap(&mut snake_segment.0, &mut temp_pos);
        }
    }
}
fn project_positions(mut block_position: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut block_position {
        transform.translation = position.0.extend(0.);
    }
}

fn project_board(
    board: Res<GameBoard>,
    mut block_position: Query<(&mut Transform, &GridLocation)>,
) {
    let total_board_width = board.width as f32 * board.cell_size;
    let total_board_height = board.height as f32 * board.cell_size;
    let left_offset = -total_board_width / 2.0;
    let top_offset = -total_board_height / 2.0;

    for (mut transform, position) in &mut block_position {
        transform.translation.x = left_offset + (position.0.x * board.cell_size);
        transform.translation.y = top_offset + (position.0.y * board.cell_size);
        transform.translation.z = 1.0;
        println!("left offset = {:?}", left_offset);
        println!("top offset = {:?}", top_offset);
        println!("position x = {:?}", left_offset * position.0.x);
        println!("position y = {:?}", left_offset * position.0.y);
    }
}
/*
fn change_speed( keyboard_input: Res<ButtonInput<KeyCode>>,mut time: ResMut<Time<Fixed>>){
    if keyboard_input.pressed(KeyCode::Space) {
        time.set_timestep(Duration::from_secs(1));
    }
}
*/

fn draw_board(
    board: Res<GameBoard>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let total_board_width = board.width as f32 * board.cell_size;
    let total_board_height = board.height as f32 * board.cell_size;
    let left_offset = -total_board_width / 2.0;
    let top_offset = -total_board_height / 2.0;

    let shape = Rectangle::new(20.0, 20.0);
    let mesh = meshes.add(shape);
    //let colour = Color::Srgba(GREEN);
    let green_material = materials.add(Color::Srgba(GREEN));
    let light_green_material = materials.add(Color::Srgba(LIGHT_GREEN));
    /*
        println!("window width = {:?}", window.width());
        println!("total board width = {:?}", total_board_width);

        println!("window height = {:?}", window.height());
        println!("total board height = {:?}", total_board_height);

        println!("left offset = {:?}", left_offset);
        println!("top offset = {:?}", top_offset);
    */
    for x in 0..board.width {
        for y in 0..board.height {
            commands.spawn((
                SnakeHead,
                Mesh2d(mesh.clone()),
                /*
                if (x + y) % 2 == 0 {
                    MeshMaterial2d(light_green_material.clone())
                } else {
                    MeshMaterial2d(green_material.clone())
                },*/
                MeshMaterial2d(light_green_material.clone()),
                Position(Vec2::new(
                    (left_offset / 2.0) + (x as f32 * 20.0),
                    top_offset + (y as f32 * 20.0),
                )),
            ));
        }
    }
}
fn spawn_apple(mut commands: Commands, mut next_state: ResMut<NextState<FoodState>>) {
    let mut rng = rand::rng();
    let apple_x_pos = rng.random_range(0..=10) as f32;
    let apple_y_pos = rng.random_range(0..=10) as f32;
    let colour = Color::Srgba(RED);
    commands.spawn((
        Apple,
        GridLocation(Vec2::new(apple_x_pos, apple_y_pos)),
        Sprite {
            color: colour,
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
    ));
    next_state.set(FoodState::OnTheBoard);
}
