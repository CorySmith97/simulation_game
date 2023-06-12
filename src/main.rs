use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};

mod player;
pub use player::*;
mod ui;
pub use ui::*;

mod name;
pub use name::*;

mod ants;
pub use ants::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(EguiPlugin)
        .add_plugin(PanCamPlugin::default())
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(print_name_system)
        .add_system(ant_wander_system)
        .run();
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let background_image = asset_server.load("ant_map.png");
    commands.spawn(Camera2dBundle::default()).insert(PanCam {
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: background_image.clone(),
        transform: Transform::from_scale(Vec3::splat(4.)),
        ..Default::default()
    });
    let texture_handle = asset_server.load("ant.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(64.0, 64.0),
        1,
        1,
        Some(Vec2::new(1., 1.)),
        None,
    );
    let ant_sprite_sheet = SpriteSheetBundle {
        texture_atlas: texture_atlases.add(texture_atlas),
        ..Default::default()
    };
    
    for _ in 1..10 {
        let ant = AntBundle::new(100, 10, ant_sprite_sheet.clone());
        commands.spawn(ant);
    }

    let texture_handle = asset_server.load("smiley_face.png");
    let queen_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(64.0, 64.0),
        1,
        1,
        Some(Vec2::new(1., 1.)),
        None,
    );       
    let queen_transform = Transform::from_translation(Vec3{x: -1500., y: -1500., z:1.})
        .with_scale(Vec3::splat(6.0));

    let queen_sprite = SpriteSheetBundle {
        texture_atlas: texture_atlases.add(queen_atlas),
        transform: queen_transform,
        ..Default::default()
    };

    let queen = QueenBundle::new(20, 500, queen_sprite);
    commands.spawn(queen);
}
