use bevy::app::{App, Plugin, PreStartup};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::color::palettes::basic::GREEN;
use bevy::color::palettes::css::LIGHT_GREEN;
use bevy::math::Vec2;
use bevy::prelude::{ColorMaterial, Commands, Mesh, Mesh2d, MeshMaterial2d, Rectangle, Res, ResMut, Resource, Update};
use crate::components::Position;

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
        app.add_systems(Update, draw_board);
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
fn draw_board(
    board: Res<GameBoard>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let total_board_width = board.width() as f32 * board.cell_size();
    let total_board_height = board.height() as f32 * board.cell_size();
    let left_offset = -total_board_width / 2.0;
    let top_offset = -total_board_height / 2.0;

    let shape = Rectangle::new(20.0, 20.0);
    let mesh = meshes.add(shape);
    //let colour = Color::Srgba(GREEN);
    let green_material = materials.add(Color::Srgba(GREEN));
    let light_green_material = materials.add(Color::Srgba(LIGHT_GREEN));
    /*
        info!("window width = {:?}", window.width());
        info!("total board width = {:?}", total_board_width);

        info!("window height = {:?}", window.height());
        info!("total board height = {:?}", total_board_height);

        info!("left offset = {:?}", left_offset);
        info!("top offset = {:?}", top_offset);
    */

    for x in 0..board.width() {
        for y in 0..board.height() {
            commands.spawn((
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