use bevy::color::palettes::css::BLUE;
use bevy::prelude::*;


#[derive(Component)]
struct Block;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,(spawn_camera,spawn_block))
        .add_systems(Update,(move_block))
        .run();
}
fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2d);
}
fn spawn_block(  mut commands: Commands,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<ColorMaterial>>,){
    let shape = Rectangle::new(20.0,20.0);
    let colour = Color::Srgba(BLUE);
    let mesh = meshes.add(shape);
    let material = materials.add(colour);
    commands.spawn((Block, Mesh2d(mesh),MeshMaterial2d(material),Transform::from_xyz(0.0,0.0,0.0)));

}

fn move_block(  mut block: Query<&mut Transform, With<Block>>,  keyboard_input: Res<ButtonInput<KeyCode>>, ){
    if let Ok(mut transform) = block.single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.y = 100.;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.y = -100.;
        } else {
            transform.translation.y = 0.;
        }
    }
}