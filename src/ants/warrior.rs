use bevy::prelude::*;
use super::{AntHealth, Ant, RizzPoints};


#[derive(Component)]
pub struct WarriorAnt {
    ant: Ant,
    name: String,   
    sprite: SpriteBundle,
    position: (f32, f32),
    health: AntHealth,
    //rizz: RizzPoints,
}


impl WarriorAnt {
    pub fn new(name: String, health: u32, sprite: SpriteBundle, position: (f32, f32)) -> WarriorAnt {
        WarriorAnt { ant: Ant {}, name, sprite, position, health: AntHealth::new(health) }
    }
}