use bevy::app::{App, Plugin, Startup};
use bevy::color::Color;
use bevy::prelude::{default, Commands, JustifyText, Node, PositionType, Text, TextColor, TextFont, TextLayout, Val};

pub struct ScorePlugin;

impl Plugin for ScorePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_scoreboard);
    }
}
fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        Text::new("Score"),
        TextFont {
            font_size: 50.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(20.0),
            ..default()
        },
    ));
}
