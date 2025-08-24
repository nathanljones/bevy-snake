use crate::components::GridLocation;
use crate::plugins::game_board::GameBoard;
use bevy::app::{App, Plugin, Startup};
use bevy::color::palettes::basic::YELLOW;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Component, Res, Sprite};

#[derive(Component)]
#[require(Sprite)]
pub struct SnakeSegment;
impl Plugin for SnakeSegment {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_snake_body);
    }
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