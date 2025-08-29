use crate::components::{GridLocation, Position};
use crate::plugins::game_board::GameBoard;
use bevy::app::{App, Plugin, Update};
use bevy::log::info;
use bevy::prelude::{Component, Query, Res, Transform};

#[derive(Component)]
pub struct ProjectionPlugin;
impl Plugin for ProjectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, project_positions);
        app.add_systems(Update, project_board);
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
        info!("left offset = {:?}", left_offset);
        info!("top offset = {:?}", top_offset);
        info!("position x = {:?}", left_offset * position.0.x);
        info!("position y = {:?}", left_offset * position.0.y);
    }
}
