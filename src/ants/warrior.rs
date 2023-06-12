use bevy::prelude::*;
use super::{AntHealth, Ant, RizzPoints};


#[derive(Component)]
pub struct WarriorAnt {
    ant: Ant,
    name: String,   
    sprite: SpriteBundle,
    position: (f32, f32),
    health: AntHealth,
    rizz: RizzPoints,
}