use bevy::app::{App, Plugin, PreStartup};
use bevy::prelude::{Commands, Resource};

#[derive(Resource)]
pub struct GameBoard {
    width: u32,
    height: u32,
    cell_size: f32,
}
impl GameBoard {
    pub fn new(width: u32, height: u32, cell_size: f32) -> Self {
        Self {
            width,
            height,
            cell_size,
        }
    }
    pub fn default() -> Self {
        Self {
            width: 10,
            height: 10,
            cell_size: 20.0,
        }
    }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cell_size(&self) -> f32 {
        self.cell_size
    }
}
impl Plugin for GameBoard {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_board);
    }
}
fn setup_board(mut commands: Commands) {
    let board = GameBoard {
        width: 10,
        height: 10,
        cell_size: 20.0,
    };
    commands.insert_resource(board);
}
