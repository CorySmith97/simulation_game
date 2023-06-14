use bevy::prelude::*;
use super::{AntHealth, Ant, RizzPoints, MoveCooldown};
use rand::Rng;


#[derive(Bundle)]
pub struct WarriorAnt {
    pub ant: Ant,
    pub sprite: SpriteSheetBundle,
    pub movement: MoveCooldown,
    //position: (f32, f32),
    pub health: AntHealth,
    //rizz: RizzPoints,
}


impl WarriorAnt {
    pub fn new( health: u32, sprite: SpriteSheetBundle) -> WarriorAnt {
        WarriorAnt { 
            ant: Ant {},
            sprite: sprite, 
            movement: MoveCooldown{
                direction: Vec2 { x: 1., y: 1. },
                speed: 30.,
                time: Timer::from_seconds(0.5, TimerMode::Repeating),
                moving: false},
            health: AntHealth::new(health) 
        }
    }
}

pub fn birth_warrior_ants(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
)  {
    let texture_handle = asset_server.load("blue_hex.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(64.0, 64.0),
        1,
        1,
        Some(Vec2::new(1., 1.)),
        None,
    );
    let mut rand_thread = rand::thread_rng();
    let x = rand_thread.gen_range(-200.0..200.00);
    let y = rand_thread.gen_range(-200.0..200.00);
        
    let ant_sprite_sheet = SpriteSheetBundle {
        texture_atlas: texture_atlases.add(texture_atlas.clone()),
        transform: Transform::from_xyz(x -1500., y -1500., 1.),
        ..Default::default()
    };
    let ant = WarriorAnt::new(100, ant_sprite_sheet.clone());
    commands.spawn(ant);
}

