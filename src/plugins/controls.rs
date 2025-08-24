use bevy::input::ButtonInput;
use bevy::prelude::{App, KeyCode, Plugin, Res, ResMut, Resource, Update};
pub struct ControlsPlugin;

#[derive(Default)]
pub enum SnakeDirection {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Resource, Default)]
pub struct CurrentSnakeDirection {
    pub snake_direction: SnakeDirection,
}

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentSnakeDirection>();
        app.add_systems(Update, change_snake_direction);
    }
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
