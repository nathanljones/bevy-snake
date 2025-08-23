use bevy::prelude::EventWriter;
use bevy::prelude::Update;
use bevy::prelude::{default, Commands, EventReader, Sprite};
use bevy::app::{App, Plugin, Startup};
use bevy::color::Color;
use bevy::color::palettes::basic::RED;
use bevy::math::Vec2;
use bevy::prelude::{Component, Event};
use rand::Rng;
use crate::components::GridLocation;
#[derive(Component)]
#[require(Sprite)]
pub struct Apple;

#[derive(Event)]
pub struct AppleEaten;

impl Plugin for Apple {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initial_spawn_apple);
        app.add_systems(Update, spawn_apple);
    }

}
fn spawn_apple(mut commands: Commands, mut events: EventReader<AppleEaten>) {
    for _event in events.read() {
        let mut rng = rand::rng();
        let apple_x_pos = rng.random_range(0..=10) as f32;
        let apple_y_pos = rng.random_range(0..=10) as f32;
        let colour = Color::Srgba(RED);
        commands.spawn((
            Apple,
            GridLocation(Vec2::new(apple_x_pos, apple_y_pos)),
            Sprite {
                color: colour,
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
        ));
    }
}
fn initial_spawn_apple(mut events: EventWriter<AppleEaten>) {
    events.write(AppleEaten);
}
