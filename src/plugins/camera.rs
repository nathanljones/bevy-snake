use bevy::prelude::*;
pub struct MainCamera;
impl Plugin for MainCamera {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}