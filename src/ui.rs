use bevy::prelude::*;
use super::health::*;
use bevy::input::mouse::{MouseButtonInput, MouseWheel};
use core::slice::Windows;

#[derive(Component)]
pub struct Text;

pub fn spawn_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&AntHealth>,
) {
    let health = query.single();
       commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                health.to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 60.0,
                color: Color::GOLD,
            }),
        ]),
        Text,
    ));
}
