use bevy::prelude::*;
use bevy::input::mouse::{MouseButtonInput, MouseWheel};
use core::slice::Windows;
use bevy_egui::{egui, EguiContexts};
use crate::ants::*;

pub fn ui_health_query(query: Query<&AntHealth>, mut contexts: EguiContexts) {
    let health = query.single();
    egui::Window::new("Health").show(contexts.ctx_mut(), |ui| {
        ui.label(health.to_string());
    });
}

pub fn click_sprite_open_health_window(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut query: Query<&mut AntHealth>,
    mut contexts: EguiContexts,
) {
    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left {
            let mut health = query.single_mut();
            egui::Window::new("Health").show(contexts.ctx_mut(), |ui| {
                ui.label(health.to_string());
            });
        }
    }
}
