use bevy::prelude::*;
use super::health::*;
use bevy::input::mouse::{MouseButtonInput, MouseWheel};
use core::slice::Windows;

pub fn mouse_pressed(
    commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    query: Query<(Entity, &mut AntHealth)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        Health_Query(commands, query);
    }       
}

pub fn mouse_right_pressed(
    commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    query: Query<(Entity, &mut AntHealth)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        damage_ant(commands, query);
    }       
}

