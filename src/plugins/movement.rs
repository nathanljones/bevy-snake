use crate::components::GridLocation;
use crate::plugins::controls::{CurrentSnakeDirection, SnakeDirection};
use crate::plugins::snake_body::SnakeSegment;
use crate::plugins::snake_head::SnakeHead;
use bevy::app::{App, Update};
use bevy::prelude::{IntoScheduleConfigs, Plugin, Query, Res, With, Without};
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            move_snake.run_if(on_timer(Duration::from_millis(150))),
        );
    }
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
