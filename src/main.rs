use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_egui::{egui,EguiPlugin, EguiContexts};

mod player;
pub use player::*;
mod health;
pub use health::*;
mod ui;
pub use ui::*;
mod ants;
pub use ants::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugin(EguiPlugin)
    .add_plugin(PanCamPlugin::default())
    .add_startup_system(setup)
    .add_system(move_player)
    .add_system(ui_example_system)
    .add_system(ant_wander_system)
    .add_system(press_a_to_damage_ant)
    .run();
}

pub fn setup(mut commands: Commands, asset_server : Res<AssetServer>, mut texture_atlases : ResMut<Assets<TextureAtlas>>, mut materials : ResMut<Assets<ColorMaterial>>)
{
    let background_image = asset_server.load("ant_map.png");
    commands.spawn(Camera2dBundle::default())
        .insert(PanCam {
            grab_buttons : vec![MouseButton::Middle],
            ..Default::default()
        });
    commands.spawn(SpriteBundle {
        texture : background_image.clone(),
        transform : Transform::from_scale(Vec3::splat(4.)),
        ..Default::default()
    });
    let texture_handle = asset_server.load("ant.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(64.0,64.0),
        1,
        1,
        Some(Vec2::new(1.,1.)),
        None,
    );
    let ant1 = 
        AntBundle {
            health : AntHealth::new(100),
            sprite : SpriteSheetBundle {
                texture_atlas : texture_atlases.add(texture_atlas),
                transform : Default::default(),
                ..Default::default()
            },
        };
    commands.spawn(ant1);
}

pub fn ui_example_system(query: Query<&AntHealth>, mut contexts: EguiContexts) {
    let health = query.single();
    egui::Window::new("Health").show(contexts.ctx_mut(), |ui| {
        ui.label(health.to_string());
    });
}

