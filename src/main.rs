use bevy::prelude::*;

pub mod player;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_system(player::move_player)
    .add_startup_system(setup)
<<<<<<< Updated upstream
    .run();
}

pub fn setup(mut commands: Commands, asset_server : Res<AssetServer>, mut texture_atlases : ResMut<Assets<TextureAtlas>>)
{
    let texture_handle = asset_server.load("ant.png");
=======
    //.add_system(move_player)
    .add_system(ui_example_system)
    .add_system(ant_wander_system)
    .add_system(press_a_to_damage_ant)
    .run();
}

pub fn setup(mut commands: Commands, asset_server : Res<AssetServer>, mut texture_atlases : ResMut<Assets<TextureAtlas>>, materials : ResMut<Assets<ColorMaterial>>)
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
    let texture_handle = asset_server.load("ant_worker.png");
>>>>>>> Stashed changes
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(64.0,64.0),
        1,
        1,
        Some(Vec2::new(1.,1.)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteSheetBundle{
        texture_atlas:texture_atlas_handle,
        transform:Transform::from_scale(Vec3::splat(1.0)),
        ..default()})
    .insert(player::PlayerMovement{speed:10.});
}
