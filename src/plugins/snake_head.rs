use crate::components::GridLocation;
use crate::plugins::game_board::GameBoard;
use bevy::app::{App, Plugin, Startup};
use bevy::color::Color;
use bevy::color::palettes::basic::BLUE;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Res, Sprite, default};

#[derive(Component)]
#[require(Sprite)]
pub struct SnakeHead;

pub struct SnakeHeadPlugin;
impl Plugin for SnakeHeadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_snake_head);
    }
}
fn spawn_snake_head(board: Res<GameBoard>, mut commands: Commands) {
    let colour = Color::Srgba(BLUE);
    commands.spawn((
        SnakeHead,
        GridLocation(Vec2::new(5., 5.)),
        Sprite {
            color: colour,
            custom_size: Some(Vec2::new(board.cell_size, board.cell_size)),
            ..default()
        },
    ));
}
