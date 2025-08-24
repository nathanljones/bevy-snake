use bevy::app::{App, Plugin};
use bevy::log::info;
use bevy::prelude::{Commands, Entity, EventWriter, Query, Update, With};
use crate::components::GridLocation;
use crate::plugins::apple::{Apple, AppleEaten};
use crate::plugins::snake_head::SnakeHead;
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,check_if_snake_has_eaten_apple);
    }
}
fn check_if_snake_has_eaten_apple(
    mut commands: Commands,
    apple: Query<(Entity, &GridLocation), With<Apple>>,
    snake_head: Query<&GridLocation, With<SnakeHead>>,
    mut events: EventWriter<AppleEaten>,
) {
    info!("Checking if snake has eaten apple");
    if let Ok(snake_head_location) = snake_head.single() {
        for (apple_entity, apple_position) in &apple {
            if apple_position.0 == snake_head_location.0 {
                commands.entity(apple_entity).despawn();
                events.write(AppleEaten);
            }
        }
    }
}