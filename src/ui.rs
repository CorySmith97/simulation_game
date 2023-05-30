use bevy::prelude::*;
use super::health::*;

pub fn mouse_pressed(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<(Entity, &mut AntHealth)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        Health_Query(commands, query);
    }       
}

pub fn mouse_right_pressed(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<(Entity, &mut AntHealth)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        damage_ant(commands, query);
    }       
}
