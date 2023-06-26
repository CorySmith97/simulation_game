use bevy::sprite::SpriteBundle;

use super::{AntHealth, Rizz};

#[derive(Component)]
pub struct WorkerAnt {
    name: String,
    sprite: SpriteBundle,
    position: (f32, f32),
    health: AntHealth, // or "Health" with bundle?
    inventory: f32,
    rizz: Rizz
    base_speed: f32,
    weight_multiplier: f32,
    position: Vec2,
    carrying_food: bool
}
