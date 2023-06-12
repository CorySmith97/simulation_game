use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct WarriorAnt {
    name: String,   
    sprite: SpriteBundle,
    health: AntHealth,
    rizz: rizz
}