use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_snake::components::GridLocation;
use bevy_snake::plugins::apple::Apple;
use bevy_snake::plugins::apple::AppleEaten;
use bevy_snake::plugins::camera::MainCamera;
use bevy_snake::plugins::collision::CollisionPlugin;
use bevy_snake::plugins::controls::ControlsPlugin;
use bevy_snake::plugins::controls::CurrentSnakeDirection;
use bevy_snake::plugins::controls::SnakeDirection;
use bevy_snake::plugins::game_board::GameBoard;
use bevy_snake::plugins::projections::Projection;
use bevy_snake::plugins::snake_body::SnakeSegment;
use bevy_snake::plugins::snake_head::SnakeHead;
use std::time::Duration;

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
        .add_plugins(SnakeSegment)
        .add_plugins(Apple)
        .add_plugins(Projection)
        .add_plugins(ControlsPlugin)
        .add_plugins(CollisionPlugin)
        .add_event::<AppleEaten>()
        .add_systems(
            Startup,
            spawn_scoreboard,
        )
        //.add_systems(Update, change_speed)
        .add_systems(
            Update,
            move_snake.run_if(on_timer(Duration::from_millis(150))),
        )
        .insert_resource(Time::<Fixed>::from_seconds(2.0))
        .run();
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
