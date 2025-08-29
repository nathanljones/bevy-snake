use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_snake::plugins::apple::ApplePlugin;
use bevy_snake::plugins::camera::MainCameraPlugin;
use bevy_snake::plugins::collision::CollisionPlugin;
use bevy_snake::plugins::controls::ControlsPlugin;
use bevy_snake::plugins::game_board::GameBoardPlugin;
use bevy_snake::plugins::movement::MovementPlugin;
use bevy_snake::plugins::projections::{ProjectionPlugin};
use bevy_snake::plugins::score::ScorePlugin;
use bevy_snake::plugins::snake_body::SnakeBodyPlugin;
use bevy_snake::plugins::snake_head::SnakeHeadPlugin;

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
        .add_plugins(MainCameraPlugin)
        .add_plugins(GameBoardPlugin)
        .add_plugins(SnakeHeadPlugin)
        .add_plugins(SnakeBodyPlugin)
        .add_plugins(ApplePlugin)
        .add_plugins(ProjectionPlugin)
        .add_plugins(ControlsPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(ScorePlugin)
        //.add_systems(Update, change_speed)
        .insert_resource(Time::<Fixed>::from_seconds(2.0))
        .run();
}

/*
fn change_speed( keyboard_input: Res<ButtonInput<KeyCode>>,mut time: ResMut<Time<Fixed>>){
        if keyboard_input.pressed(KeyCode::Space) {
        time.set_timestep(Duration::from_secs(1));
    }
}
*/
