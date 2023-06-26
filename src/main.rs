use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
use rand::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_tweening::*;

mod ui;
pub use ui::*;
mod ants;
pub use ants::*;

const TIME_STEP: f32 = 1./60.;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(EguiPlugin)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(TweeningPlugin)
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(AntPlugin)
        .add_plugin(PersonalUIPlugin)
        .add_startup_system(setup)
        .add_system(my_cursor_system)
        .add_system(move_ant.system())
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
    
    //queen test 
    let texture_handle = asset_server.load("temp_queen.png");
    let queen_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(64.0, 64.0),
        1,
        1,
        Some(Vec2::new(1., 1.)),
        None,
    );       
    let queen_transform = Transform::from_translation(Vec3{x: -1500., y: -1500., z:1.})
        .with_scale(Vec3::splat(4.0));

    let queen_sprite = SpriteSheetBundle {
        texture_atlas: texture_atlases.add(queen_atlas),
        transform: queen_transform,
        ..Default::default()
    };
    let queen_name = Name::new("Mommy");
    let queen = QueenBundle::new(queen_name, 20, 500, queen_sprite);
    commands.spawn(queen);


    //warrior test
    let w_texture_handle = asset_server.load("ant_warrior.png");
    let texture_atlas = TextureAtlas::from_grid(
        w_texture_handle,
        Vec2::new(64.0, 64.0),
        1,
        1,
        Some(Vec2::new(1., 1.)),
        None,
    ); 

    let warrior_transform = Transform::from_translation(Vec3{x: 00., y: 00., z:2.})
        .with_scale(Vec3::splat(2.0));


    let warrrior_sprite = SpriteSheetBundle {
        texture_atlas: texture_atlases.add(texture_atlas),
        transform: warrior_transform,
        ..Default::default()
    };

    let test_name = String::from("test");
    let warrior = WarriorAnt::new(150, warrrior_sprite);

    commands.spawn(warrior);

    // Load the WorkerAnt sprite
    let ant_texture = asset_server.load("assets\worker_ant.png");

    // Create a new sprite using the ant texture
    let ant_sprite = Sprite::new(Vec2::new(64.0, 64.0)); // Adjust the size as needed
    let ant_material = materials.add(ant_texture.into());

    // Spawn the ant entity with the sprite
    commands.spawn_bundle(SpriteBundle {
        sprite: ant_sprite,
        material: ant_material,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    })
    .insert(WorkerAnt {
        position: Vec2::new(0.0, 0.0),
        velocity: Vec2::new(1.0, 0.0), // Set an initial velocity
    });

}
// System to move WorkerAnt based on its velocity.
fn move_ant(time: Res<Time>, mut query: Query<(&mut Ant, &mut Transform)>) {
    for (mut ant, mut transform) in query.iter_mut() {
        ant.position += ant.velocity * time.delta_seconds();
        transform.translation = ant.position.extend(0.0);
    }
}
