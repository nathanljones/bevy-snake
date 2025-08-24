use bevy::color::palettes::css::{BLUE, GREEN, LIGHT_GREEN, YELLOW};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_snake::components::{GridLocation, Position};
use bevy_snake::plugins::apple::Apple;
use bevy_snake::plugins::apple::AppleEaten;
use bevy_snake::plugins::camera::MainCamera;
use bevy_snake::plugins::game_board::GameBoard;
use bevy_snake::plugins::projections::Projection;
use bevy_snake::plugins::snake_head::SnakeHead;
use std::time::Duration;


#[derive(Component)]
#[require(Sprite)]
struct SnakeSegment;

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
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Snake".into(),
                    ..default()
                }),
                ..default()
            })
            .set(LogPlugin {
                filter: "error,bevy_snake=trace".to_string(),
                level: bevy::log::Level::TRACE,
                ..default()
            }),))
        .add_plugins(MainCamera)
        .add_plugins(GameBoard::default())
        .add_plugins(SnakeHead)
        .add_plugins(Apple)
        .add_plugins(Projection)
        .init_resource::<CurrentSnakeDirection>()
        .add_event::<AppleEaten>()
        .add_systems(
            Startup,
            (spawn_snake_body, spawn_scoreboard),
        )
        .add_systems(
            Update,
            (
                change_snake_direction,
                check_if_snake_has_eaten_apple,
            ),
        )
        //.add_systems(Update, change_speed)
        .add_systems(
            Update,
            move_snake.run_if(on_timer(Duration::from_millis(150))),
        )
        .insert_resource(Time::<Fixed>::from_seconds(2.0))
        .run();
}


fn spawn_snake_body(board: Res<GameBoard>, mut commands: Commands) {
    let colour = Color::Srgba(YELLOW);
    commands.spawn((
        SnakeSegment,
        GridLocation(Vec2::new(5., -4.)),
        Sprite {
            color: colour,
            custom_size: Some(Vec2::new(
                board.cell_size() as f32,
                board.cell_size() as f32,
            )),
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

/*
fn change_speed( keyboard_input: Res<ButtonInput<KeyCode>>,mut time: ResMut<Time<Fixed>>){
        if keyboard_input.pressed(KeyCode::Space) {
        time.set_timestep(Duration::from_secs(1));
    }
}
*/



fn check_if_snake_has_eaten_apple(
    mut commands: Commands,
    apple: Query<(Entity, &GridLocation), With<Apple>>,
    snake_head: Query<&GridLocation, With<SnakeHead>>,
    mut events: EventWriter<AppleEaten>,
) {
    info!("Checking if snake has eaten apple");
    if let Ok(snake_head_location) = snake_head.single() {
        for (apple_entity, apple_position) in &apple {
            if apple_position.0 == snake_head_location.0 {
                commands.entity(apple_entity).despawn();
                events.write(AppleEaten);
            }
        }
    }
}

fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        Text::new("Score"),
        TextFont {
            font_size: 50.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(20.0),
            ..default()
        },
    ));
}
