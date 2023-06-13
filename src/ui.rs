use bevy::prelude::*;
use bevy::input::mouse::{MouseButtonInput, MouseWheel};
use bevy_egui::{egui, EguiContexts, EguiSettings};
use crate::ants::*;
use bevy::sprite::MaterialMesh2dBundle;


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

pub fn main_bottom_menu(    
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut contexts: EguiContexts,
    mut egui_settings: ResMut<EguiSettings>,
    rizz: Query<&mut RizzPoints>,
    )
{
    let ctx = contexts.ctx_mut();
    
    egui_settings.scale_factor = 1.3;
    egui::TopBottomPanel::bottom("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
            ui.separator();

            egui::menu::menu_button(ui, "Spawn", |ui| {
                if ui.button("Spawn Ant").clicked() {
                    birth_ants(commands, asset_server, texture_atlases, keyboard_input);
                }
            });
            ui.separator();

            egui::menu::menu_button(ui, "Rizz", |ui| {
                if ui.button("Check Rizz").clicked() {
                    query_rizz(rizz);
                }
            })

            
        });
    });

}

pub fn my_cursor_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut ant_q: Query<(&Ant, &mut Transform)>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();
    
    for (ant, mut transform) in ant_q.iter_mut(){
        let ant_l = transform.translation.truncate();
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            if (ant_l.x.ceil() == world_position.x.ceil() && ant_l.y.ceil() == world_position.y.ceil()) {
                println!("WOO");
            }
        }
    }
}
