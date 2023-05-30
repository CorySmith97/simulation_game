use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

mod player;
pub use player::*;
mod health;
pub use health::*;
mod ui;
pub use ui::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugin(PanCamPlugin::default())
    .add_startup_system(setup)
    .add_system(move_player)
    .add_system(mouse_pressed)
    .add_system(mouse_right_pressed)
    .run();
}

pub fn setup(mut commands: Commands, asset_server : Res<AssetServer>, mut texture_atlases : ResMut<Assets<TextureAtlas>>, mut materials : ResMut<Assets<ColorMaterial>>)
{
    let background_image = asset_server.load("ant_map.png");
    commands.spawn(Camera2dBundle::default())
        .insert(PanCam::default());
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
    commands.spawn(SpriteSheetBundle {
        texture_atlas : texture_atlases.add(texture_atlas),
        transform : Transform::from_scale(Vec3::splat(4.)),
        ..Default::default()
    }).insert(AntHealth::new(100));

}

