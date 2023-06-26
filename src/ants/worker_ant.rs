use bevy::sprite::SpriteBundle;

use super::{AntHealth, Rizz};

#[derive(Component)]
pub struct WorkerAnt {
    name: String,
    sprite: SpriteBundle,
    position: Vec2,
    health: AntHealth,
    inventory: f32,
    rizz: Rizz,
    velocity: Vec2,
    weight_multiplier: f32,
    position: Vec2,
    carrying_food: bool
}
